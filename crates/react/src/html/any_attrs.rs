use std::collections::HashMap;
use std::hash::Hash;

pub trait AnyAttributeValue {
    type Key: Hash + Eq;

    fn as_attribute_key(&self) -> Self::Key;
}

pub struct AnyAttributes<V: AnyAttributeValue>(HashMap<V::Key, V>);

impl<V: AnyAttributeValue> AnyAttributes<V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn set(&mut self, v: V) -> &mut Self {
        let map = &mut self.0;
        let k = v.as_attribute_key();
        map.insert(k, v);
        self
    }

    pub fn get(&self, key: &V::Key) -> Option<&V> {
        self.0.get(key)
    }
}
