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

