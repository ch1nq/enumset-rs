use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;
use std::fmt;

// A wrapper for an Enum, that hashes using only the variant type of the enum   
struct EnumWrapper<T>(T);

impl<T> PartialEq for EnumWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(&self.0) == discriminant(&other.0)
    }
}

impl<T> Eq for EnumWrapper<T> {}

impl<T> Hash for EnumWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}

impl<T: fmt::Debug> fmt::Debug for EnumWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct EnumSet<T> {
    inner_set: HashSet<EnumWrapper<T>>
} 

impl<T: fmt::Debug> fmt::Debug for EnumSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.debug_struct("EnumSet")
         .finish()
         .and(self.inner_set.fmt(f))
    }
}

impl<T> EnumSet<T> {
    fn key_from_variant<V: Default>(&self, variant: fn(V) -> T) -> EnumWrapper<T> {
        // Set data of the enum variant to the default value, 
        // as only the variant of the enum is used as a key in the HashSet
        EnumWrapper(variant(V::default()))
    }

    pub fn update(&mut self, item: T) {
        self.inner_set.replace(EnumWrapper(item));
    }

    pub fn remove<V: Default>(&mut self, variant: fn(V) -> T) -> bool {
        let key = self.key_from_variant(variant);
        self.inner_set.remove(&key)
    }

    pub fn get<V: Default>(&self, variant: fn(V) -> T) -> Option<&T> {
        let key = self.key_from_variant(variant);
        match self.inner_set.get(&key) {
            Some(EnumWrapper(item)) => Some(item),
            _ => None
        }
    }

    /// Returns whether a given variant is in the set or not
    pub fn exists(&self, variant: T) -> bool {
        self.inner_set.get(&EnumWrapper(variant)).is_some()
    }
}

impl<T> Default for EnumSet<T> {
    fn default() -> Self {
        Self {
            inner_set: HashSet::default()
        }
    }
}
