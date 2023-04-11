use crate::{BinTree, BinTreeIter, BinTreeIterMut};

/// node type for BinTreeMap
#[derive(Debug,Clone,Default)]
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
    /// transforms the map into the wrapped binary tree
    pub fn into_inner(self) -> BinTree<BinTreeMapKeyVal<Key,Value>> {
        self.data
    }
    /// returns a ref to the wrapped binary tree
    pub fn inner(&self) -> &BinTree<BinTreeMapKeyVal<Key,Value>> {
        &self.data
    }
    pub fn to_tree_string(&self) -> String where Key: std::fmt::Debug, Value : std::fmt::Debug {
        format!("{}",self.inner())
    }
    /// iter for BinTreeMap
    pub fn iter(&self) -> BinTreeMapIter<'_, Key, Value> {
        BinTreeMapIter{iter:self.data.iter()}
    }
    /// iter_mut for BinTreeMap
    pub fn iter_mut(&mut self) -> BinTreeMapIterMut<'_, Key, Value> {
        BinTreeMapIterMut{iter:self.data.iter_mut()}
    }
}

impl<Key : PartialOrd + Default, Value: Default> BinTreeMap<Key,Value> {
    /// remove by key from the map and return removed value
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        if let Some(kv) = self.data.remove_sorted_with_key(key, &BinTreeMapKeyVal::key) {
            Some(kv.value)
        } else {
            None
        }
    }
}

impl<Key : PartialOrd + std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for BinTreeMap<Key,Value> {
    /// display a map as a vector of tuples
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data.iter().map(|kv| (&kv.key, &kv.value)).collect::<Vec<_>>())
    }
}

/// iter for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIter<'a,K,V> where K : PartialOrd {
    iter: BinTreeIter<'a,BinTreeMapKeyVal<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIter<'a,K,V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapKeyVal{key,value}) = self.iter.next() {
            Some((key,value))
        } else {
            None
        }
    }
}

/// iter_mut for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIterMut<'a,K,V> where K : PartialOrd {
    iter: BinTreeIterMut<'a,BinTreeMapKeyVal<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIterMut<'a,K,V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapKeyVal{key,value}) = self.iter.next() {
            Some((key,value))
        } else {
            None
        }
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTreeMap};

    #[test]
    fn test_map() {
        let mut map = BinTreeMap::new();

        map.insert(67, String::from("first value for 67"));
        map.insert(31, String::from("value for 31"));
        map.insert(67, String::from("second value for 67"));

        assert_eq!(map.iter().collect::<Vec<_>>(),
            [(&31, &String::from("value for 31")), (&67, &String::from("second value for 67"))]);

        for (k,v) in map.iter_mut() {
            if *k < 50 {
                *v = format!("{} is less than 50",v);
            }
        }

        assert_eq!(map.to_string(),"[(31, \"value for 31 is less than 50\"), (67, \"second value for 67\")]");
        assert_eq!(*map.get(&31).unwrap(),"value for 31 is less than 50");
        assert_eq!(map.get_key_value(&67).unwrap(),(&67,&String::from("second value for 67")));
        assert_eq!(map.contains_key(&67),true);
        assert_eq!(map.contains_key(&167),false);

        assert_eq!(format!("{:?}",map),
            "BinTreeMap { data: BinTree { root: Some(\
                BinTreeNode { \
                    value: BinTreeMapKeyVal { key: 67, value: \"second value for 67\" }, \
                    left: BinTree { root: Some(\
                        BinTreeNode { \
                            value: BinTreeMapKeyVal { key: 31, value: \"value for 31 is less than 50\" }, \
                            left: BinTree { root: None }, \
                            right: BinTree { root: None } \
                        }) }, \
                    right: BinTree { root: None } \
                }) } }"
            );
        assert_eq!(map.to_tree_string(),
            "((\
                BinTreeMapKeyVal { key: 31, value: \"value for 31 is less than 50\" }) <= \
                BinTreeMapKeyVal { key: 67, value: \"second value for 67\" }\
            )");
        assert_eq!(map.remove(&31).unwrap(),"value for 31 is less than 50");
        assert_eq!(map.to_string(),"[(67, \"second value for 67\")]");
    }
}
