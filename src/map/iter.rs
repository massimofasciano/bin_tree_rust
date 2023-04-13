use crate::{BinTreeMap, BinTreeIter, BinTreeIterMut, BinTreeIntoIter, BinTreeMapEntry};

impl<Key : PartialOrd, Value> BinTreeMap<Key,Value> {
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
