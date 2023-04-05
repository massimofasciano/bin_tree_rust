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
    pub fn into_inner(self) -> BinTree<Item> {
        self.data
    }
}

impl<Item: PartialOrd> Extend<Item> for OrderedSetBinTree<Item> {
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<Item : PartialOrd + std::fmt::Display> std::fmt::Display for OrderedSetBinTree<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<Item : PartialOrd> FromIterator<Item> for OrderedSetBinTree<Item> {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut s = Self::default();
        iter.into_iter().for_each(|e| s.insert(e));
        s
    }
}

impl<T : PartialOrd> IntoIterator for OrderedSetBinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<Item : PartialOrd> PartialEq for OrderedSetBinTree<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|e| other.contains(e)) &&
        other.iter().all(|e| self.contains(e))
    }
}

impl<Item : PartialOrd> From<BinTree<Item>> for OrderedSetBinTree<Item> {
    fn from(value: BinTree<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> From<Vec<Item>> for OrderedSetBinTree<Item> {
    fn from(value: Vec<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> Into<BinTree<Item>> for OrderedSetBinTree<Item> {
    fn into(self) -> BinTree<Item> {
        self.into_inner()
    }
}

#[cfg(test)]
mod test {
    use crate::{OrderedSetBinTree, BinTree};

    static TEST_STR : &str = "Hello, my name is Joe!";

    #[test]
    fn test_basic() {
        let str1 = "(((  => (!)) <= ,) <= H => (((J) <= a) <= e => ((i) <= l => ((m => (n)) <= o => ((s) <= y)))))";
        let mut s = OrderedSetBinTree::new();
        TEST_STR.chars().for_each(|c| s.insert(c));
        assert_eq!(s.to_string(),str1);
        let s2 = TEST_STR.chars().collect::<OrderedSetBinTree<_>>();
        assert_eq!(s2.to_string(),str1);
        assert_eq!(s,s2);
        let s3 = OrderedSetBinTree::from(TEST_STR.chars().collect::<Vec<_>>());
        assert_eq!(s3.to_string(),str1);
        assert_eq!(s3,s2);
        let s4 = OrderedSetBinTree::from(TEST_STR.chars().collect::<BinTree<_>>());
        assert_eq!(s4.to_string(),"(  => (! => (, => (H => (J => (a => (e => (i => (l => (m => (n => (o => (s => (y))))))))))))))");
        assert_eq!(s3,s4);
        TEST_STR.chars().for_each(|c| s.remove(&c));
        assert_eq!(s.to_string(),"()");
        s.extend(TEST_STR.chars());
        assert_eq!(s.to_string(),str1);
    }
}
