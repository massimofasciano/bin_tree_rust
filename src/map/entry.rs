/// node entry for BinTreeMap
#[derive(Debug,Clone,Default)]
pub struct BinTreeMapEntry<Key,Value> where Key : PartialOrd {
    pub key: Key,
    pub value: Value,
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

