use crate::{BinTree};

#[derive(Debug,Clone)]
pub struct BinTreeMapKeyVal<Key,Value> where Key : PartialOrd {
    key: Key,
    value: Value,
}

/// a basic ordered set container shows how to encapsulate a type inside another
#[derive(Debug,Clone)]
#[repr(transparent)]
pub struct BinTreeMap<Key,Value> where Key : PartialOrd {
    data: BinTree<BinTreeMapKeyVal<Key,Value>>
}

/// default set is an empty tree
impl<Key : PartialOrd, Value> Default for BinTreeMap<Key,Value> {
    fn default() -> Self {
        Self {
            data: BinTree::default()
        }
    }
}

impl<Key : PartialOrd, Value> BinTreeMapKeyVal<Key,Value> {
    fn new(key: Key, value : Value) -> Self {
        Self { key, value }
    }
    fn key(&self) -> &Key {
        &self.key
    }
}

impl<Key : PartialOrd, Value> BinTreeMap<Key,Value> {
    /// empty tree
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: Key, value: Value) {
        self.data.push_key_sorted_unique(BinTreeMapKeyVal::new(key,value),BinTreeMapKeyVal::key);
    }
}

impl<Key : PartialOrd + std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for BinTreeMap<Key,Value> {
    /// display a map as a vector of tuples
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data.iter().map(|kv| (&kv.key, &kv.value)).collect::<Vec<_>>())
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTreeMap};

    #[test]
    fn test_map() {
        let mut map = BinTreeMap::new();

        map.insert(67, "first value for 67");
        map.insert(31, "value for 31");
        map.insert(67, "second value for 67");

        assert_eq!(map.to_string(),"[(31, \"value for 31\"), (67, \"second value for 67\")]");
    }
}
