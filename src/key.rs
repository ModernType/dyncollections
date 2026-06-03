use std::marker::PhantomData;

/// This struct represents a key to the value in [`OrdDynMap`] or [`HashDynMap`] with knowledge about specific type `T` stored behind the key.
/// Creating a [`DynKey`] or overwriting contained type is unsafe and can lead to *undefined behavior* as it can lead to retrieval of trait object and interpreting
/// it as an incorrect type.
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
    /// Binds provided key to particular value. Binding to wrong type can lead to *undefined behavior*.
    /// SAFETY: Caller must guarantee that the value in collection behind this key is of type `T`.
    pub unsafe fn new(key: K) -> Self {
        Self {
            key,
            _type: PhantomData,
        }
    }

    /// Rebinds this key to a different type. Binding to wrong type can lead to *undefined behavior*.
    /// SAFETY: Caller must guarantee that the value in collection behind this key is of type `O`.
    pub unsafe fn cast<O>(self) -> DynKey<K, O> {
        DynKey {
            key: self.key,
            _type: PhantomData,
        }
    }

    /// Returns the underlying key value.
    pub fn key(&self) -> &K {
        &self.key
    }
}
