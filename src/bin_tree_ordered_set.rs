use crate::{BinTree, BinTreeIntoIter, BinTreeIter};

/// a basic ordered set container shows how to encapsulate a type inside another
#[derive(Debug,Clone)]
pub struct OrderedSetBinTree<Item> where Item : PartialOrd {
    data: BinTree<Item>,
    len: usize,
}

/// default set is an empty tree
impl<Item : PartialOrd> Default for OrderedSetBinTree<Item> {
    fn default() -> Self {
        Self {
            data: BinTree::default(),
            len: 0
        }
    }
}

impl<Item : PartialOrd> OrderedSetBinTree<Item> {
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
        if self.data.push_sorted_unique(value) {
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

impl<Item: PartialOrd> Extend<Item> for OrderedSetBinTree<Item> {
    /// extend a set from an iterator
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<Item : PartialOrd + std::fmt::Debug> std::fmt::Display for OrderedSetBinTree<Item> {
    /// display a set as a vector
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data.iter().collect::<Vec<_>>())
    }
}

impl<Item : PartialOrd> FromIterator<Item> for OrderedSetBinTree<Item> {
    /// create a set from an iterator
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut s = Self::default();
        iter.into_iter().for_each(|e| s.insert(e));
        s
    }
}

impl<T : PartialOrd> IntoIterator for OrderedSetBinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    /// set into_iter (depth-first in-order tree into_iter)
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter_dfs_in()
    }
}

impl<Item : PartialOrd> PartialEq for OrderedSetBinTree<Item> {
    /// set equality
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|e| other.contains(e)) &&
        other.iter().all(|e| self.contains(e))
    }
}

impl<Item : PartialOrd> From<BinTree<Item>> for OrderedSetBinTree<Item> {
    /// create set from binary tree
    fn from(value: BinTree<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> From<Vec<Item>> for OrderedSetBinTree<Item> {
    /// create set from vector
    fn from(value: Vec<Item>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<Item : PartialOrd> Into<BinTree<Item>> for OrderedSetBinTree<Item> {
    /// transform set into wrapped binary tree
    fn into(self) -> BinTree<Item> {
        self.into_inner()
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{OrderedSetBinTree, BinTree};

    static TEST_STR : &str = "Hello, my name is Joe!";

    #[test]
    fn test_basic() {
        // let str1 = "(((' ' => ('!')) <= ',') <= 'H' => ((('J') <= 'a') <= \
        //     'e' => (('i') <= 'l' => (('m' => ('n')) <= 'o' => (('s') <= 'y')))))";
        let str1 = 
                    "(((' ' \
                        => ('!')) \
                <= ',' => \
                        (('H') \
                    <= 'J' => \
                        ('a'))) \
            <= 'e' => \
                    ((('i') <= 'l') \
                <= 'm' => \
                        (('n') \
                    <= 'o' => \
                            (('s') \
                        <= 'y'))))";
        let mut s = OrderedSetBinTree::new();
        TEST_STR.chars().for_each(|c| s.insert(c));
        assert_eq!(s.to_tree_string(),str1);
        assert_eq!(s.len(), 14);
        let s2 = TEST_STR.chars().collect::<OrderedSetBinTree<_>>();
        assert_eq!(s2.to_tree_string(),str1);
        assert_eq!(s2.len(), 14);
        assert_eq!(s,s2);
        let s3 = OrderedSetBinTree::from(TEST_STR.chars().collect::<Vec<_>>());
        assert_eq!(s3.to_tree_string(),str1);
        assert_eq!(s3.len(), 14);
        assert_eq!(s3,s2);
        let s4 = OrderedSetBinTree::from(TEST_STR.chars().collect::<BinTree<_>>());
        assert_eq!(s4.to_string(),"[' ', '!', ',', 'H', 'J', 'a', 'e', 'i', 'l', 'm', 'n', 'o', 's', 'y']");
        assert_eq!(s4.len(), 14);
        assert_eq!(s3,s4);
        TEST_STR.chars().for_each(|c| { s.remove(&c); } );
        assert_eq!(s.to_tree_string(),"()");
        assert_eq!(s.len(), 0);
        s.extend(TEST_STR.chars());
        assert_eq!(s.to_tree_string(),str1);
        assert_eq!(s.len(), 14);
        let str2 = s.iter().collect::<String>();
        assert_eq!(str2," !,HJaeilmnosy");
        assert_eq!(s.len(),14);
    }
}
