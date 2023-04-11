use crate::BinTree;

/// node type for BinTreeMap
#[derive(Debug,Clone)]
pub struct BinTreeMapKeyVal<Key,Value> where Key : PartialOrd {
    key: Key,
    value: Value,
}

/// a basic map container shows how to encapsulate a type inside another
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
    // fn tuple(&self) -> (&Key, &Value) {
    //     (&self.key,&self.value)
    // }
}

impl<Key : PartialOrd, Value> BinTreeMap<Key,Value> {
    /// empty tree
    pub fn new() -> Self {
        Self::default()
    }
    /// number of elements in the map
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// is the map empty ?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// insert into the map
    pub fn insert(&mut self, key: Key, value: Value) {
        self.data.push_sorted_unique_with_key(BinTreeMapKeyVal::new(key,value),&BinTreeMapKeyVal::key);
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
        if let Some(kv) = self.data.get_sorted_with_key(key, &BinTreeMapKeyVal::key) {
            Some((&kv.key,&kv.value))
        } else {
            None
        }
    }
    /// check if map contains key
    pub fn contains_key(&self, key: &Key) -> bool {
        self.get(key).is_some()
    }
    /// remove by key from the map and return removed value
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        if let Some(kv) = self.data.remove_sorted_with_key(key, &BinTreeMapKeyVal::key) {
            Some(kv.value)
        } else {
            None
        }
    }
    // pub fn iter(&self) -> () {
    //     self.data.iter().map(BinTreeMapKeyVal::tuple).into_iter()
    // }
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

        // assert_eq!(map.iter().collect::<Vec<_>>,[]);

        assert_eq!(map.to_string(),"[(31, \"value for 31\"), (67, \"second value for 67\")]");
        assert_eq!(map.get(&31),Some(&"value for 31"));
        assert_eq!(map.get_key_value(&67),Some((&67,&"second value for 67")));
        assert_eq!(map.contains_key(&67),true);
        assert_eq!(map.contains_key(&167),false);

        assert_eq!(map.remove(&31),Some("value for 31"));
        assert_eq!(map.to_string(),"[(67, \"second value for 67\")]");
    }
}
