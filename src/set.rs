use crate::{make_concrete::MakeConcrete, ordmap::OrdDynMap};

pub type DynKey<T> = crate::key::DynKey<usize, T>;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash)]
pub struct DynSet<DynT>
where
    DynT: ?Sized,
{
    inner: OrdDynMap<usize, DynT>,
    cur_id: usize,
}

impl<DynT> DynSet<DynT>
where
    DynT: ?Sized,
{
    /// Creates new empty [`DynSet`]
    pub const fn new() -> Self {
        Self {
            inner: OrdDynMap::new(),
            cur_id: 0,
        }
    }

    /// Pushes a new element into [`DynSet`]. Returns a key which provides access to its concrete type
    pub fn push<T>(&mut self, value: T) -> DynKey<T>
    where
        DynT: MakeConcrete<T>,
    {
        let id = self.cur_id;
        self.cur_id += 1;
        // SAFETY: We increment the counter constantly, so there won't be the same key we can overwrite
        unsafe { self.inner.insert_overwrite(id, value) }
    }

    /// Returns a reference to the value corresponding to the key and type bind
    pub fn get<T>(&self, key: &DynKey<T>) -> Option<&T>
    where
        DynT: MakeConcrete<T>,
    {
        self.inner.get(key)
    }

    /// Returns a mutable reference to the value corresponding to the key and type bind
    pub fn get_mut<T>(&mut self, key: &DynKey<T>) -> Option<&mut T>
    where
        DynT: MakeConcrete<T>,
    {
        self.inner.get_mut(key)
    }

    /// Returns a reference to the value corresponding to the key without concrete type
    pub fn get_dyn(&self, id: usize) -> Option<&DynT> {
        self.inner.get_dyn(&id)
    }

    /// Returns a mutable reference to the value corresponding to the key without concrete type
    pub fn get_dyn_mut(&mut self, id: usize) -> Option<&mut DynT> {
        self.inner.get_dyn_mut(&id)
    }

    /// Returns an iterator over references of entries of the map
    pub fn iter(&self) -> impl Iterator<Item = &Box<DynT>> {
        self.inner.iter().map(|(_, v)| v)
    }

    /// Returns an iterator over mutable references of entries of the map
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<DynT>> {
        self.inner.iter_mut().map(|(_, v)| v)
    }

    /// Returns whether the set is empty (i.e. contains no elements)
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes the element with the given key from the set, returning its concrete type value if found
    pub fn remove<T>(&mut self, key: &DynKey<T>) -> Option<Box<T>>
    where
        DynT: MakeConcrete<T>,
    {
        self.inner.remove(key)
    }

    /// Removes the element with the given id from the set, returning corresponding trait object
    pub fn remove_dyn(&mut self, id: usize) -> Option<Box<DynT>> {
        self.inner.remove_dyn(&id)
    }

    /// Removes all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<DynT: ?Sized> IntoIterator for DynSet<DynT> {
    type IntoIter = DynSetIter<DynT>;
    type Item = Box<DynT>;

    fn into_iter(self) -> Self::IntoIter {
        DynSetIter {
            btree_iter: self.inner.into_iter(),
        }
    }
}

pub struct DynSetIter<DynT: ?Sized> {
    btree_iter: std::collections::btree_map::IntoIter<usize, Box<DynT>>,
}

impl<DynT: ?Sized> Iterator for DynSetIter<DynT> {
    type Item = Box<DynT>;

    fn next(&mut self) -> Option<Self::Item> {
        self.btree_iter.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.btree_iter.size_hint()
    }
}

#[cfg(test)]
mod test {
    use crate::{dynamify, set::DynSet};

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

    #[test]
    fn test_push_and_get() {
        let mut dynset: DynSet<dyn Test> = DynSet::new();
        let hello_key = dynset.push(Hello);
        let world_key = dynset.push(World);
        assert_eq!(
            dynset.get(&hello_key).unwrap().hello_own_fn(),
            "Hello, I'm concrete `Hello` instance"
        );
        assert_eq!(
            dynset.get(&world_key).unwrap().world_own_fn(),
            "World, I'm concrete `World` instance"
        );
    }

    #[test]
    fn test_iter() {
        let mut dynset: DynSet<dyn Test> = DynSet::new();
        let _hello_key = dynset.push(Hello);
        let _world_key = dynset.push(World);

        let messages: Vec<_> = dynset.iter().map(|i| i.message()).collect();
        assert_eq!(messages, vec!["Hello", "World"]);
    }
}
