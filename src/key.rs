use std::marker::PhantomData;

#[derive(Debug)]
pub struct DynKey<K, T> {
    key: K,
    _type: PhantomData<T>,
}

impl<K, T> Clone for DynKey<K, T>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _type: PhantomData,
        }
    }
}

impl<K, T> Copy for DynKey<K, T> where K: Copy {}

impl<K, T> DynKey<K, T> {
    /// Binds provided key to particular value. Binding to wrong type can lead to *undefined behavior*
    pub unsafe fn new(key: K) -> Self {
        Self {
            key,
            _type: PhantomData,
        }
    }

    /// Rebinds this key to a different type. Binding to wrong type can lead to *undefined behavior*
    pub unsafe fn cast<O>(self) -> DynKey<K, O> {
        DynKey {
            key: self.key,
            _type: PhantomData,
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }
}
