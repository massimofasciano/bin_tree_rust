use crate::{BinTree, BinTreeIter, BinTreeIterMut, BinTreeIntoIter};

/// node type for BinTreeMap
#[derive(Debug,Clone,Default)]
pub struct BinTreeMapEntry<Key,Value> where Key : PartialOrd {
    key: Key,
    value: Value,
}

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

impl<Key : PartialOrd, Value> BinTreeMapEntry<Key,Value> {
    /// get the key from a BinTreeMapEntry
    pub fn key(&self) -> &Key {
        &self.key
    }
}
impl<Key : PartialOrd, Value> PartialEq for BinTreeMapEntry<Key,Value> {
    /// equality for BinTreeMapEntry (by key)
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<Key : PartialOrd, Value> PartialOrd for BinTreeMapEntry<Key,Value> {
    /// partial order for BinTreeMapEntry (by key)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
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
        self.len += 1;
        self.data.push_sorted_unique(BinTreeMapEntry{key,value});
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
    /// iter for BinTreeMap
    pub fn iter(&self) -> BinTreeMapIter<'_, Key, Value> {
        BinTreeMapIter{iter:self.data.iter()}
    }
    /// iter_mut for BinTreeMap
    pub fn iter_mut(&mut self) -> BinTreeMapIterMut<'_, Key, Value> {
        BinTreeMapIterMut{iter:self.data.iter_mut()}
    }
    /// keys for BinTreeMap
    pub fn keys(&self) -> BinTreeMapIterKeys<'_, Key, Value> {
        BinTreeMapIterKeys{iter:self.data.iter()}
    }
    /// values for BinTreeMap
    pub fn values(&self) -> BinTreeMapIterValues<'_, Key, Value> {
        BinTreeMapIterValues{iter:self.data.iter()}
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

impl<Key: PartialOrd,Value> IntoIterator for BinTreeMap<Key,Value> {
    type IntoIter = BinTreeMapIntoIter<Key,Value>;
    type Item = (Key, Value);

    fn into_iter(self) -> Self::IntoIter {
        BinTreeMapIntoIter{iter:self.data.into_iter()}
    }
}

/// into_iter for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIntoIter<K,V> where K : PartialOrd {
    iter: BinTreeIntoIter<BinTreeMapEntry<K,V>>
}

impl<K : PartialOrd,V> Iterator for BinTreeMapIntoIter<K,V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapEntry{key,value}) = self.iter.next() {
            Some((key,value))
        } else {
            None
        }
    }
}

impl<'a,Key: PartialOrd,Value> IntoIterator for &'a BinTreeMap<Key,Value> {
    type IntoIter = BinTreeMapIter<'a,Key,Value>;
    type Item = (&'a Key, &'a Value);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// iter for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIter<'a,K,V> where K : PartialOrd {
    iter: BinTreeIter<'a,BinTreeMapEntry<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIter<'a,K,V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapEntry{key,value}) = self.iter.next() {
            Some((key,value))
        } else {
            None
        }
    }
}

impl<'a,Key: PartialOrd,Value> IntoIterator for &'a mut BinTreeMap<Key,Value> {
    type IntoIter = BinTreeMapIterMut<'a,Key,Value>;
    type Item = (&'a Key, &'a mut Value);

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// iter_mut for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIterMut<'a,K,V> where K : PartialOrd {
    iter: BinTreeIterMut<'a,BinTreeMapEntry<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIterMut<'a,K,V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapEntry{key,value}) = self.iter.next() {
            Some((key,value))
        } else {
            None
        }
    }
}

/// keys iter for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIterKeys<'a,K,V> where K : PartialOrd {
    iter: BinTreeIter<'a,BinTreeMapEntry<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIterKeys<'a,K,V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapEntry{key,value: _}) = self.iter.next() {
            Some(key)
        } else {
            None
        }
    }
}

/// values iter for BinTreeMap (uses BinTree iterator)
#[repr(transparent)]
pub struct BinTreeMapIterValues<'a,K,V> where K : PartialOrd {
    iter: BinTreeIter<'a,BinTreeMapEntry<K,V>>
}

impl<'a,K : PartialOrd,V> Iterator for BinTreeMapIterValues<'a,K,V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BinTreeMapEntry{key: _,value}) = self.iter.next() {
            Some(value)
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

        for (k,v) in &mut map {
            if *k < 50 {
                *v = format!("{} is less than 50",v);
            }
        }

        assert_eq!(map.to_string(),"[(31, \"value for 31 is less than 50\"), (67, \"second value for 67\")]");
        assert_eq!(*map.get(&31).unwrap(),"value for 31 is less than 50");
        assert_eq!(map.get_key_value(&67).unwrap(),(&67,&String::from("second value for 67")));
        assert_eq!(map.contains_key(&67),true);
        assert_eq!(map.contains_key(&167),false);

        assert_eq!(map.to_tree_string(),
            "((\
                BinTreeMapEntry { key: 31, value: \"value for 31 is less than 50\" }) <= \
                BinTreeMapEntry { key: 67, value: \"second value for 67\" }\
            )");
        assert_eq!(map.remove(&31).unwrap(),"value for 31 is less than 50");
        assert_eq!(map.to_string(),"[(67, \"second value for 67\")]");

        assert_eq!(map.into_iter().collect::<Vec<_>>(),vec![(67, "second value for 67".to_owned())]);
    }

    #[test]
    fn check_custom_kv_type() {
        #[derive(PartialEq, PartialOrd, Debug, Default)]
        struct KeyType(i32);

        #[derive(Debug, Default, PartialEq)]
        struct ValueType(i64);

        let mut t : BinTreeMap<KeyType, ValueType> = BinTreeMap::new();

        assert_eq!(t.len(),0);
        t.insert(KeyType(-20), ValueType(782));
        assert_eq!(t.len(),1);
        t.insert(KeyType(3330), ValueType(-1782));
        assert_eq!(t.len(),2);
        t.insert(KeyType(33), ValueType(14));
        assert_eq!(t.len(),3);
        t.insert(KeyType(110), ValueType(-1));
        assert_eq!(t.len(),4);
        t.insert(KeyType(-40), ValueType(234));
        assert_eq!(t.len(),5);
        t.insert(KeyType(12), ValueType(82));
        assert_eq!(t.len(),6);
        t.insert(KeyType(130), ValueType(-2));
        assert_eq!(t.len(),7);
        t.insert(KeyType(-876), ValueType(-182));
        assert_eq!(t.len(),8);

        assert_eq!(t.to_tree_string(),"\
                (((BinTreeMapEntry { key: KeyType(-876), value: ValueType(-182) }) <= \
                BinTreeMapEntry { key: KeyType(-40), value: ValueType(234) }) <= \
                BinTreeMapEntry { key: KeyType(-20), value: ValueType(782) } => \
                (((BinTreeMapEntry { key: KeyType(12), value: ValueType(82) }) <= \
                BinTreeMapEntry { key: KeyType(33), value: ValueType(14) } => \
                (BinTreeMapEntry { key: KeyType(110), value: ValueType(-1) } => \
                (BinTreeMapEntry { key: KeyType(130), value: ValueType(-2) }))) <= \
                BinTreeMapEntry { key: KeyType(3330), value: ValueType(-1782) }))\
            ");

        assert_eq!(t.remove(&KeyType(12)).unwrap(),ValueType(82));
        assert_eq!(t.len(),7);

        assert_eq!(t.to_tree_string(),"\
                (((BinTreeMapEntry { key: KeyType(-876), value: ValueType(-182) }) <= \
                BinTreeMapEntry { key: KeyType(-40), value: ValueType(234) }) <= \
                BinTreeMapEntry { key: KeyType(-20), value: ValueType(782) } => \
                ((BinTreeMapEntry { key: KeyType(33), value: ValueType(14) } => \
                (BinTreeMapEntry { key: KeyType(110), value: ValueType(-1) } => \
                (BinTreeMapEntry { key: KeyType(130), value: ValueType(-2) }))) <= \
                BinTreeMapEntry { key: KeyType(3330), value: ValueType(-1782) }))\
            ");

        assert_eq!(format!("{:?}",t.keys().collect::<Vec<_>>()),
            "[KeyType(-876), KeyType(-40), KeyType(-20), KeyType(33), KeyType(110), KeyType(130), KeyType(3330)]");
        assert_eq!(format!("{:?}",t.values().collect::<Vec<_>>()),
            "[ValueType(-182), ValueType(234), ValueType(782), ValueType(14), ValueType(-1), ValueType(-2), ValueType(-1782)]");

        *t.get_mut(&KeyType(110)).unwrap() = ValueType(-110);
        assert_eq!(t.len(),7);
        assert_eq!(t.into_iter().collect::<Vec<_>>(),vec![
            (KeyType(-876), ValueType(-182)), 
            (KeyType(-40), ValueType(234)), 
            (KeyType(-20), ValueType(782)), 
            (KeyType(33), ValueType(14)), 
            (KeyType(110), ValueType(-110)), 
            (KeyType(130), ValueType(-2)), 
            (KeyType(3330), ValueType(-1782))]);
    }
}
