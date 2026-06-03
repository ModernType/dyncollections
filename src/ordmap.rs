use crate::{key::DynKey, make_concrete::MakeConcrete};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrdDynMap<K, DynT>
where
    DynT: ?Sized,
{
    inner: BTreeMap<K, Box<DynT>>,
}

impl<K, DynT> OrdDynMap<K, DynT>
where
    DynT: ?Sized,
{
    pub const fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}

impl<K, DynT> OrdDynMap<K, DynT>
where
    DynT: ?Sized,
    K: Ord + Clone,
{
    /// Tries to insert a new value with provided key. If value with this key exists, returns `Err` with value, otherwise `Ok` containing the key.
    pub fn insert<T>(&mut self, key: K, value: T) -> Result<DynKey<K, T>, T>
    where
        DynT: MakeConcrete<T>,
    {
        if self.inner.contains_key(&key) {
            return Err(value);
        }
        self.inner.insert(key.clone(), DynT::from_concrete(value));
        // SAFETY: We accepted this type in function signature
        unsafe { Ok(DynKey::new(key)) }
    }

    /// Inserts the value with provided key, potentially overwriting previous value which can make earlier keys hold the wrong type.
    /// This method can be more performant than standard `insert`.
    ///
    /// SAFETY: You must ensure that you won't acces this value with old key with incorrect binded type
    pub unsafe fn insert_overwrite<T>(&mut self, key: K, value: T) -> DynKey<K, T>
    where
        DynT: MakeConcrete<T>,
    {
        self.inner.insert(key.clone(), DynT::from_concrete(value));
        // SAFETY: We accepted this type in function signature
        unsafe { DynKey::new(key) }
    }

    /// Returns a reference to the value corresponding to the key and type bind
    pub fn get<T>(&self, key: &DynKey<K, T>) -> Option<&T>
    where
        DynT: MakeConcrete<T>,
    {
        let dyn_value = self.inner.get(key.key())?;
        // SAFETY: DynKey associated type is a guarantee that this conversion must be valid
        Some(unsafe { dyn_value.as_concrete() })
    }

    /// Returns a mutable reference to the value corresponding to the key and type bind
    pub fn get_mut<T>(&mut self, key: &DynKey<K, T>) -> Option<&mut T>
    where
        DynT: MakeConcrete<T>,
    {
        let dyn_value = self.inner.get_mut(key.key())?;
        // SAFETY: DynKey associated type is a guarantee that this conversion must be valid
        Some(unsafe { dyn_value.as_concrete_mut() })
    }

    /// Returns a reference to the value corresponding to the key without concrete type
    pub fn get_dyn(&self, key: &K) -> Option<&DynT> {
        self.inner.get(key).map(|v| v.deref())
    }

    /// Returns a mutable reference to the value corresponding to the key without concrete type
    pub fn get_dyn_mut(&mut self, key: &K) -> Option<&mut DynT> {
        self.inner.get_mut(key).map(|v| v.deref_mut())
    }

    /// Returns an iterator over references of entries of the map
    pub fn iter(&self) -> impl Iterator<Item = (&K, &Box<DynT>)> {
        self.inner.iter()
    }

    /// Returns an iterator over mutable references of entries of the map
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut Box<DynT>)> {
        self.inner.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove<T>(&mut self, key: &DynKey<K, T>) -> Option<Box<T>>
    where
        DynT: MakeConcrete<T>,
    {
        let dyn_value: Box<DynT> = self.inner.remove(key.key())?;
        // SAFETY: The key provides us with type information for conversion.
        unsafe {
            let ref_value = &mut *Box::into_raw(dyn_value);
            let ref_value = DynT::as_concrete_mut(ref_value) as *mut T;
            Some(Box::from_raw(ref_value))
        }
    }

    pub fn remove_dyn(&mut self, key: &K) -> Option<Box<DynT>> {
        self.inner.remove(key)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
}

impl<K, DynT: ?Sized> IntoIterator for OrdDynMap<K, DynT> {
    type Item = (K, Box<DynT>);
    type IntoIter = std::collections::btree_map::IntoIter<K, Box<DynT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::OrdDynMap;
    use crate::dynamify;

    #[test]
    fn test_valid_value() {
        trait Test {
            fn message(&self) -> &'static str;
        }

        #[derive(Debug)]
        struct Hello;
        impl Hello {
            fn hello_own_fn(&self) -> &'static str {
                "Hello, I'm concrete `Hello` instance"
            }
        }
        impl Test for Hello {
            fn message(&self) -> &'static str {
                "Hello"
            }
        }

        #[derive(Debug)]
        struct World;
        impl World {
            fn world_own_fn(&self) -> &'static str {
                "World, I'm concrete `World` instance"
            }
        }
        impl Test for World {
            fn message(&self) -> &'static str {
                "World"
            }
        }

        dynamify!(Test);

        let mut dynmap: OrdDynMap<usize, dyn Test> = OrdDynMap::new();
        let hello_key = dynmap.insert(0, Hello).unwrap();
        let world_key = dynmap.insert(1, World).unwrap();

        assert_eq!(
            dynmap.get(&hello_key).unwrap().hello_own_fn(),
            "Hello, I'm concrete `Hello` instance"
        );
        assert_eq!(
            dynmap.get(&world_key).unwrap().world_own_fn(),
            "World, I'm concrete `World` instance"
        );
    }
}
