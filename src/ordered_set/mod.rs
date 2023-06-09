use crate::{BinTree, BinTreeIntoIter, BinTreeIter};

/// a basic ordered set container shows how to encapsulate a type inside another
#[derive(Debug,Clone)]
pub struct BinTreeOrderedSet<Item> where Item : PartialOrd {
    data: BinTree<Item>,
    len: usize,
}

/// default set is an empty tree
impl<Item : PartialOrd> Default for BinTreeOrderedSet<Item> {
    fn default() -> Self {
        Self {
            data: BinTree::default(),
            len: 0
        }
    }
}

impl<Item : PartialOrd> BinTreeOrderedSet<Item> {
    /// empty tree
    pub fn new() -> Self {
        Self::default()
    }
    /// number of elements in the set
    pub fn len(&self) -> usize {
        self.len
    }
    /// set insertion (uses push_ordered_unique tree method)
    pub fn insert(&mut self, value : Item) {
        if self.data.insert_unique(value) {
            self.len += 1;
        }
    }
    /// remove from set (uses remove_sorted tree method)
    pub fn remove(&mut self, value : &Item) -> Option<Item> where Item : Default {
        if let Some(removed) = self.data.remove_sorted(value) {
            self.len -= 1;
            Some(removed)
        } else {
            None
        }
    }
    /// find a value in the set (uses contains_sorted tree method)
    pub fn contains(&self, value : &Item) -> bool {
        self.data.contains_sorted(value)
    }
    /// set iterator (depth-first in-order tree iterator)
    pub fn iter(&self) -> BinTreeIter<Item> {
        self.data.iter_dfs_in()
    }
    /// transforms the set into the wrapped binary tree
    pub fn into_inner(self) -> BinTree<Item> {
        self.data
    }
    /// returns a ref to the wrapped binary tree
    pub fn inner(&self) -> &BinTree<Item> {
        &self.data
    }
    pub fn to_tree_string(&self) -> String where Item : std::fmt::Debug {
        format!("{}",self.inner())
    }
}

impl<Item: PartialOrd> Extend<Item> for BinTreeOrderedSet<Item> {
    /// extend a set from an iterator
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<Item : PartialOrd + std::fmt::Debug> std::fmt::Display for BinTreeOrderedSet<Item> {
    /// display a set as a vector
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data.iter().collect::<Vec<_>>())
    }
}

impl<Item : PartialOrd> FromIterator<Item> for BinTreeOrderedSet<Item> {
    /// create a set from an iterator
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut s = Self::default();
        iter.into_iter().for_each(|e| s.insert(e));
        s
    }
}

impl<T : PartialOrd> IntoIterator for BinTreeOrderedSet<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    /// set into_iter (depth-first in-order tree into_iter)
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter_dfs_in()
    }
}

impl<Item : PartialOrd> PartialEq for BinTreeOrderedSet<Item> {
    /// set equality
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|e| other.contains(e)) &&
        other.iter().all(|e| self.contains(e))
    }
}

impl<Item : PartialOrd> From<BinTree<Item>> for BinTreeOrderedSet<Item> {
    /// create set from binary tree
    fn from(value: BinTree<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> From<Vec<Item>> for BinTreeOrderedSet<Item> {
    /// create set from vector
    fn from(value: Vec<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> Into<BinTree<Item>> for BinTreeOrderedSet<Item> {
    /// transform set into wrapped binary tree
    fn into(self) -> BinTree<Item> {
        self.into_inner()
    }
}

#[cfg(test)]
mod test;
