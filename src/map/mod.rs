use crate::{BinTree};

/// methods that use bits of unsafe code
pub mod utils_unsafe;
pub use self::utils_unsafe::*;

/// node entry for BinTreeMap
pub mod entry;
pub use self::entry::*;

/// iterators for BinTreeMap
pub mod iter;
pub use self::iter::*;

/// a basic map implementation using BinTree
#[derive(Debug,Clone)]
pub struct BinTreeMap<Key,Value> where Key : PartialOrd {
    data: BinTree<BinTreeMapEntry<Key,Value>>,
    len: usize,
}

/// default set is an empty tree
impl<Key : PartialOrd, Value> Default for BinTreeMap<Key,Value> {
    fn default() -> Self {
        Self {
            data: BinTree::default(),
            len: 0,
        }
    }
}

impl<Key : PartialOrd, Value> BinTreeMap<Key,Value> {
    /// empty tree
    pub fn new() -> Self {
        Self::default()
    }
    /// number of elements in the map
    pub fn len(&self) -> usize {
        self.len
    }
    /// is the map empty ?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// insert into the map
    pub fn insert(&mut self, key: Key, value: Value) {
        if self.data.push_sorted_unique(BinTreeMapEntry{key,value}) {
            self.len += 1;
        }
    }
    /// get a value by key from the map
    pub fn get(&self, key: &Key) -> Option<&Value> {
        if let Some((_, value)) = self.get_key_value(key) {
            Some(value)
        } else {
            None
        }
    }
    /// get (key,value) by key from the map
    pub fn get_key_value(&self, key: &Key) -> Option<(&Key, &Value)> {
        if let Some(kv) = self.data.get_sorted_with_key(key, &BinTreeMapEntry::key) {
                Some((&kv.key,&kv.value))
        } else {
            None
        }
    }
    /// get mut value by key from the map
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        if let Some(kv) = self.data.get_sorted_mut_with_key(key, &&BinTreeMapEntry::key) {
            Some(&mut kv.value)
        } else {
            None
        }
    }
    /// check if map contains key
    pub fn contains_key(&self, key: &Key) -> bool {
        self.get(key).is_some()
    }
    /// transforms the map into the wrapped binary tree
    pub fn into_inner(self) -> BinTree<BinTreeMapEntry<Key,Value>> {
        self.data
    }
    /// returns a ref to the wrapped binary tree
    pub fn inner(&self) -> &BinTree<BinTreeMapEntry<Key,Value>> {
        &self.data
    }
    pub fn to_tree_string(&self) -> String where Key: std::fmt::Debug, Value : std::fmt::Debug {
        format!("{}",self.inner())
    }
}

impl<Key : PartialOrd + Default, Value: Default> BinTreeMap<Key,Value> {
    /// remove by key from the map and return removed value
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        if let Some(kv) = self.data.remove_sorted_with_key(key, &BinTreeMapEntry::key) {
            self.len -= 1;
            Some(kv.value)
        } else {
            None
        }
    }
}

impl<Key: PartialOrd + std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for BinTreeMap<Key,Value> {
    /// display a map as a vector of tuples
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data.iter().map(|kv| (&kv.key, &kv.value)).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod test;