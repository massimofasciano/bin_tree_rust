use crate::{BinTree, BinTreeIntoIter, BinTreeIter, BinTreeIterMut};

#[derive(Debug,Clone)]
#[repr(transparent)]
pub struct OrderedSetBinTree<Item> where Item : PartialOrd {
    data: BinTree<Item>
}

impl<Item : PartialOrd> Default for OrderedSetBinTree<Item> {
    fn default() -> Self {
        Self {
            data: BinTree::default()
        }
    }
}

impl<Item : PartialOrd> OrderedSetBinTree<Item> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, value : Item) {
        self.data.push_sorted_unique(value);
    }
    pub fn remove(&mut self, value : &Item) {
        self.data.remove_sorted(value);
    }
    pub fn contains(&self, value : &Item) -> bool {
        self.data.contains_sorted(value)
    }
    pub fn iter(&self) -> BinTreeIter<Item> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> BinTreeIterMut<Item> {
        self.data.iter_mut()
    }
}

impl<Item : PartialOrd + std::fmt::Display> std::fmt::Display for OrderedSetBinTree<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<Item : PartialOrd> FromIterator<Item> for OrderedSetBinTree<Item> {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        Self {
            data: BinTree::from_iter(iter)
        }
    }
}

impl<T : PartialOrd> IntoIterator for OrderedSetBinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<Item : PartialOrd + Clone> PartialEq for OrderedSetBinTree<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|e| other.contains(e)) &&
        other.iter().all(|e| self.contains(e))
    }
}

#[cfg(test)]
mod test {
    use crate::OrderedSetBinTree;

    #[test]
    fn test_basic() {
        let mut s = OrderedSetBinTree::new();
        "Hello, my name is Joe!".chars().for_each(|c| s.insert(c));
        assert_eq!(s.to_string(),"(((  => (!)) <= ,) <= H => (((J) <= a) <= e => ((i) <= l => ((m => (n)) <= o => ((s) <= y)))))");
        let s2 = "Hello, my name is Joe!".chars().collect::<OrderedSetBinTree<_>>();
        assert_eq!(s,s2);
        "Hello, my name is Joe!".chars().for_each(|c| s.remove(&c));
        assert_eq!(s.to_string(),"()");
    }
}
