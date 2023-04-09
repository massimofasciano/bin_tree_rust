use std::ops::Deref;

/// a general purpose binary tree
#[derive(Debug,Clone,PartialEq)]
pub struct BinTree<Item> {
    root: Option<Box<BinTreeNode<Item>>>
}

/// a general purpose binary tree node
#[derive(Debug,Clone,PartialEq)]
pub struct BinTreeNode<Item> {
    value : Item,
    left : BinTree<Item>,
    right : BinTree<Item>,
}

impl<Item> BinTree<Item> {
    /// creates a branch
    pub fn new_branch(value : Item, left: BinTree<Item>, right: BinTree<Item>) -> Self {
        Self { root : Some(Box::new(BinTreeNode{value, left, right}))}
    }
    /// creates a leaf
    pub fn new_leaf(item : Item) -> Self {
        Self::new_branch(item, Self::new(), Self::new())
    }
    /// creates an empty tree
    pub fn new() -> Self {
        Self { root : None }
    }
    /// tests if tree is a branch (leaf is excluded although it is stored as a branch with empty children internally)
    pub fn is_branch(&self) -> bool {
        !self.is_empty() &&
        !self.is_leaf()
    }
    /// tests if tree is a leaf (leaf is stored as a branch with empty children internally)
    pub fn is_leaf(&self) -> bool {
        !self.is_empty() &&
        self.left().unwrap().is_empty() &&
        self.right().unwrap().is_empty()
    }
    /// tests if tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    /// returns a ref to the 3 elements of the branch at the top of the tree (if not empty)
    pub fn branch(&self) -> Option<(&Item,&BinTree<Item>,&BinTree<Item>)> {
        if let Some(node) = self.root {
            let BinTreeNode { value, left, right } = node.as_ref();
            Some((value,left,right))
        } else {
            None
        }
    }
    /// returns a ref to the value at the top of the tree
    pub fn value(&self) -> Option<&Item> {
        if let Some(node) = self.root {
            let BinTreeNode { value, left, right } = node.as_ref();
            Some(value)
        } else {
            None
        }
    }
    /// returns the left branch of the tree
    pub fn left(&self) -> Option<&BinTree<Item>> {
        if let Some(node) = self.root {
            let BinTreeNode { value, left, right } = node.as_ref();
            Some(left)
        } else {
            None
        }
    }
    /// returns the right branch of the tree
    pub fn right(&self) -> Option<&BinTree<Item>> {
        if let Some(node) = self.root {
            let BinTreeNode { value, left, right } = node.as_ref();
            Some(right)
        } else {
            None
        }
    }
    /// returns a mutable ref to the 3 elements of the branch at the top of the tree (if not empty)
    pub fn branch_mut(&mut self) -> Option<(&mut Item,&mut BinTree<Item>,&mut BinTree<Item>)> {
        if let Some(mut node) = self.root {
            let BinTreeNode { value, left, right } = node.as_mut();
            Some((value,left,right))
        } else {
            None
        }
    }
    /// returns a mutable ref to the value at the top of the tree
    pub fn value_mut(&mut self) -> Option<&mut Item> {
        if let Some(mut node) = self.root {
            let BinTreeNode { value, left, right } = node.as_mut();
            Some(value)
        } else {
            None
        }
    }
    /// returns a mutable ref to the left branch of the tree
    pub fn left_mut(&mut self) -> Option<&mut BinTree<Item>> {
        if let Some(mut node) = self.root {
            let BinTreeNode { value, left, right } = node.as_mut();
            Some(left)
        } else {
            None
        }
    }
    /// returns a mutable ref to the right branch of the tree
    pub fn right_mut(&mut self) -> Option<&mut BinTree<Item>> {
        if let Some(mut node) = self.root {
            let BinTreeNode { value, left, right } = node.as_mut();
            Some(right)
        } else {
            None
        }
    }
}

impl<Item> Default for BinTree<Item> {
    /// default is an empty tree
    fn default() -> Self {
        Self::new()
    }
}

impl<Item : std::fmt::Debug> std::fmt::Display for BinTree<Item> {
    /// display a tree (on one line)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_line(f)
    }
}

impl<Item> BinTree<Item> {
    /// display a tree on a single line with arrows indicating branches
    pub fn write_line(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
        where Item : std::fmt::Debug 
    {
        if let Some((item, left, right)) = self.branch() {
            write!(f,"(")?;
            if !left.is_empty() {
                left.write_line(f)?;
                write!(f," <= ")?;
            }
            write!(f,"{:?}",item)?;
            if !right.is_empty() {
                write!(f," => ")?;
                right.write_line(f)?;
            }
            write!(f,")")
        } else {
            // empty tree
            write!(f,"()")
        }
    }
    /// display a tree on multiple lines with a configurable tab (indent)
    pub fn pretty_write(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result
        where Item : std::fmt::Debug 
    {
        self.pretty_write_indent(f, tab, 0)
    }
    fn pretty_write_indent(&self, f: &mut std::fmt::Formatter<'_>, tab : &str, indent : usize) -> std::fmt::Result
        where Item : std::fmt::Debug 
    {
        if let Some((item, left, right)) = self.branch() {
            right.pretty_write_indent(f, tab, indent+1)?;
            write!(f,"{}{:?}\n",tab.repeat(indent),item)?;
            left.pretty_write_indent(f, tab, indent+1)
        } else {
            // empty
            write!(f,"{}{}\n",tab.repeat(indent),"@")
        }
    }
}

/// different kinds of formatting for the tree
pub enum FormattedBinTreeType<'a> {
    PrettyIndent(&'a str),
    Line,
}

/// a container for a formatted tree
pub struct FormattedBinTree<'a, T> {
    inner: &'a BinTree<T>,
    format: FormattedBinTreeType<'a>,
}

impl<'a,T : std::fmt::Debug> std::fmt::Display for FormattedBinTree<'a,T> {
    /// display a formatted tree according to the internal format field
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            FormattedBinTreeType::Line => self.inner.write_line(f),
            FormattedBinTreeType::PrettyIndent(tab) => self.inner.pretty_write(f,tab),
        }
    }
}

impl<'a,T> FormattedBinTree<'a,T> {
    /// associate a BinTree with a format
    pub fn new(t : &'a BinTree<T>, fmt: FormattedBinTreeType<'a>) -> Self {
        Self { inner: t, format: fmt }
    }
}

impl<'a,T > Deref for FormattedBinTree<'a,T> {
    type Target = BinTree<T>;

    /// returns a ref to the inner tree
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T> From<()> for BinTree<T> {
    /// build an empty tree from the empty type
    fn from(_: ()) -> Self {
        Self::new()
    }
}

/// convenient function to construct a tree from item and branches
pub fn tree<T>(item: T, left: impl Into<BinTree<T>>, right: impl Into<BinTree<T>>) -> BinTree<T> {
    BinTree::new_branch(item, left.into(), right.into())
}

/// convenient function to construct a tree leaf from item
pub fn leaf<T>(item: T) -> BinTree<T> {
    BinTree::new_leaf(item)
}

impl<Item> BinTree<Item> {
    /// clone the contents of a tree into a vec (using default iter)
    pub fn to_vec(&self) -> Vec<Item> where Item : Clone {
        self.iter().map(|e|e.clone()).collect()
    }
}

impl<Item> Into<Vec<Item>> for BinTree<Item> {
    /// transform the tree into a vec (using default into_iter)
    fn into(self) -> Vec<Item> {
        self.into_iter().collect()
    }
}

impl<Item> BinTree<Item> {
    /// default push method (uses push_sorted)
    pub fn push(&mut self, new_item : Item) where Item : PartialOrd {
        self.push_sorted(new_item);
    }
    /// push onto a sorted or empty tree and keeps order property
    pub fn push_sorted(&mut self, new_item : Item) where Item : PartialOrd {
        if let Some((item, left, right)) = self.branch_mut() {
            if new_item < *item {
                left.push_sorted(new_item);
            } else {
                right.push_sorted(new_item);
            }
        } else {
            // empty
            *self = Self::new_leaf(new_item)
        }
    }
    /// extend a sorted or empty tree and keeps order property
    pub fn extend_sorted<T: IntoIterator<Item = Item>>(&mut self, iter: T) where Item : PartialOrd {
        for elem in iter {
            self.push_sorted(elem);
        }
    }
    /// push onto a sorted or empty tree with no duplicates and keeps both properties
    pub fn push_sorted_unique(&mut self, new_item : Item) where Item : PartialOrd {
        if let Some((item, left, right)) = self.branch_mut() {
            if new_item < *item {
                left.push_sorted_unique(new_item);
            } else if new_item > *item {
                right.push_sorted_unique(new_item);
            }
        } else {
            // empty
            *self = Self::new_leaf(new_item)
        }
    }
    /// extend a sorted or empty tree with no duplicates and keeps both properties
    pub fn extend_sorted_unique<T: IntoIterator<Item = Item>>(&mut self, iter: T) where Item : PartialOrd {
        for elem in iter {
            self.push_sorted_unique(elem);
        }
    }
    /// push to the right branch of a tree (linear tree)
    pub fn push_right(&mut self, new_item : Item) {
        if let Some((item, left, right)) = self.branch_mut() {
            right.push_right(new_item);
        } else {
            // empty
            *self = Self::new_leaf(new_item)
        }
    }
    /// extend to the right branch of a tree (linear tree)
    pub fn extend_right<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.push_right(elem);
        }
    }
    /// push to the left branch of a tree (linear tree)
    pub fn push_left(&mut self, new_item : Item) {
        if let Some((item, left, right)) = self.branch_mut() {
            left.push_left(new_item);
        } else {
            // empty
            *self = Self::new_leaf(new_item)
        }
    }
    /// extend to the left branch of a tree (linear tree)
    pub fn extend_left<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.push_left(elem);
        }
    }
    /// pop the top item from the tree
    pub fn pop(&mut self) -> Option<Item> {
        if let Some((item, left, right)) = self.branch_mut() {
            let mut p;
            p = left.pop();
            if p.is_none() {
                p = right.pop();
            }
            Some(match p {
                None => {
                    // We use unsafe to replace item with an uninit value.
                    // This is safe because we destroy self right after so this value is never read.
                    // It allows us to take Item without needing Item to implement Default.
                    let it = std::mem::replace(item, unsafe { 
                        std::mem::MaybeUninit::uninit().assume_init() 
                    });
                    *self = Self::new();
                    it
                },
                Some(it) => {
                    std::mem::replace(item, it)
                },
            })
        } else {
            // empty
            None
        }
    }
    /// returns the mutable tree node containing the minimum value item
    /// assumes that the tree is sorted
    fn min_tree_mut(&mut self) -> Option<&mut BinTree<Item>> where Item : PartialOrd {
        if self.is_leaf() {
            Some(self)
        } else if self.is_branch() {
            if self.left().unwrap().is_empty() {
                // no left path
                Some(self)
            } else {
                // min from left path
                self.left_mut().unwrap().min_tree_mut()

            }
        } else {
            None
        }
    }
    /// pop the top value from a sorted tree and preserves order
    pub fn pop_sorted(&mut self) -> Option<Item> where Item : PartialOrd {
        if let Some((item, left, right)) = self.branch_mut() {
            // When we use unsafe to replace the item with an uninit value,
            // we always destroy the current node by assigning to *self
            // so the uninitialized value is never read.
            // It allows us to take Item without needing Item to implement Default.
            if left.is_empty() && right.is_empty() {
                let it = std::mem::replace(item, unsafe { 
                    std::mem::MaybeUninit::uninit().assume_init() 
                });
                *self = Self::new();
                Some(it)
            } else if right.is_empty() {
                let it = std::mem::replace(item, unsafe { 
                    std::mem::MaybeUninit::uninit().assume_init() 
                });
                let left = std::mem::take(left);
                *self = left;
                Some(it)
            } else if left.is_empty() {
                let it = std::mem::replace(item, unsafe { 
                    std::mem::MaybeUninit::uninit().assume_init() 
                });
                let right = std::mem::take(right);
                *self = right;
                Some(it)
            } else {
                let min_right = right.min_tree_mut().expect("min right should always return some tree");
                let min_right_item = min_right.value_mut().expect("min right should always return some item");
                std::mem::swap(item,min_right_item);
                min_right.pop_sorted()
            }
        } else {
            // empty
            None
        }
    }
    /// try to remove value from a sorted tree and preserve order
    pub fn remove_sorted(&mut self, value : &Item) -> bool where Item : PartialOrd {
        if let Some((item, left, right)) = self.branch_mut() {
            if *value < *item {
                left.remove_sorted(value)
            } else if *value > *item {
                right.remove_sorted(value)
            } else {
                self.pop_sorted();
                true
            }
        } else {
            // empty
            false
        }
    }
    /// find a value in a sorted tree
    pub fn contains_sorted(&self, value : &Item) -> bool where Item : PartialOrd {
        if let Some((item, left, right)) = self.branch() {
            if *value < *item {
                left.contains_sorted(value)
            } else if *value > *item {
                right.contains_sorted(value)
            } else {
                true
            }
        } else {
            // empty
            false
        }
    }
    /// find a value in a tree (no ordering assumed)
    pub fn contains(&self, value : &Item) -> bool where Item : PartialEq {
        if let Some((item, left, right)) = self.branch() {
            value == item || 
            left.contains(value) || 
            right.contains(value)
        } else {
            // empty
            false
        }
    }
}

impl<Item: PartialOrd> Extend<Item> for BinTree<Item> {
    /// extend a tree using the default push method (ordered)
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl<Item : PartialOrd> FromIterator<Item> for BinTree<Item> {
    /// create a sorted tree from an iterator
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut t = Self::new();
        t.extend_sorted(iter);
        t
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTree, tree, leaf};

    fn test_tree() -> BinTree<i32> {
        tree(1,
            tree(2,
                leaf(3),
                ()
            ),
            tree(4,
                (),
                tree(5,
                    leaf(6),
                    ()
        )))
    }

    #[test]
    fn eq_test() {
        let bt = 
            BinTree::new_branch(1,
                BinTree::new_branch(2,
                    BinTree::new_leaf(3),
                    BinTree::new(),
                ),
                BinTree::new_branch(4,
                    BinTree::new(),
                    BinTree::new_branch(5, 
                        BinTree::new_leaf(6), 
                        BinTree::new()
                    )
                )
            );
        let t = test_tree();
        assert_eq!(t,bt);
    }

    #[test]
    fn iter_mut_test() {
        let mut t = test_tree();
        t.iter_mut().for_each(|i| {
            if *i % 2 == 1 { *i += 10 }
        });
        assert_eq!(t.to_string(),"(((13) <= 2) <= 11 => (4 => ((6) <= 15)))");
    }

    #[test]
    fn into_iter_order_test() {
        let t = test_tree();
        assert_eq!(t.into_iter().collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_in().collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_pre().collect::<Vec<_>>(),vec![1, 2, 3, 4, 5, 6]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_post().collect::<Vec<_>>(),vec![3, 2, 6, 5, 4, 1]);
        let t = test_tree();
        assert_eq!(t.into_iter_bfs().collect::<Vec<_>>(),vec![1, 2, 4, 3, 5, 6]);
    }

    #[test]
    fn iter_order_test() {
        let t = test_tree();
        assert_eq!(t.iter().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        assert_eq!(t.iter_dfs_in().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        assert_eq!(t.iter_dfs_pre().map(|i|i.clone()).collect::<Vec<_>>(),vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(t.iter_dfs_post().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 6, 5, 4, 1]);
        assert_eq!(t.iter_bfs().map(|i|*i).collect::<Vec<_>>(),vec![1, 2, 4, 3, 5, 6]);
    }

    #[test]
    fn iter_mut_order_test() {
        let mut t = test_tree();
        let mut i = 0;
        t.iter_mut().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![4, 4, 4, 8, 11, 11]);
        t.iter_mut_dfs_in().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![11, 12, 13, 18, 22, 23]);
        t.iter_mut_dfs_pre().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![26, 26, 26, 34, 40, 40]);
        t.iter_mut_dfs_post().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![45, 46, 50, 57, 61, 62]);
        t.iter_mut_bfs().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![73, 72, 75, 84, 91, 91]);
    }

    #[test]
    fn push_sorted_test() {
        let mut t = BinTree::new();
        t.push_sorted(4);
        t.push_sorted(2);
        t.push_sorted(8);
        t.push_sorted(1);
        t.push_sorted(2);
        assert_eq!(t.to_string(),"(((1) <= 2 => (2)) <= 4 => (8))");
        assert_eq!(t.to_vec(),vec![1,2,2,4,8]);
        t.extend_sorted(vec![18,6,3,8,5,11]);
        assert_eq!(t.to_string(),"(((1) <= 2 => (2 => (3))) <= 4 => (((5) <= 6) <= 8 => ((8 => (11)) <= 18)))");
        assert_eq!(t.to_vec(),vec![1,2,2,3,4,5,6,8,8,11,18]);
    }

    #[test]
    fn push_sorted_unique_test() {
        let mut t = BinTree::new();
        t.push_sorted_unique(4);
        t.push_sorted_unique(2);
        t.push_sorted_unique(8);
        t.push_sorted_unique(1);
        t.push_sorted_unique(2);
        assert_eq!(t.to_string(),"(((1) <= 2) <= 4 => (8))");
        assert_eq!(t.to_vec(),vec![1,2,4,8]);
        t.extend_sorted_unique(vec![18,6,3,8,5,11]);
        assert_eq!(t.to_string(),"(((1) <= 2 => (3)) <= 4 => (((5) <= 6) <= 8 => ((11) <= 18)))");
        assert_eq!(t.to_vec(),vec![1,2,3,4,5,6,8,11,18]);
    }

    #[test]
    fn push_right_test() {
        let mut t = BinTree::new();
        t.push_right(4);
        t.push_right(2);
        t.push_right(8);
        t.push_right(1);
        assert_eq!(t.to_string(),"(4 => (2 => (8 => (1))))");
        assert_eq!(t.to_vec(),vec![4,2,8,1]);
        t.extend_right(vec![18,6,3,5,11]);
        assert_eq!(t.to_string(),"(4 => (2 => (8 => (1 => (18 => (6 => (3 => (5 => (11)))))))))");
        assert_eq!(t.to_vec(),vec![4,2,8,1,18,6,3,5,11]);
    }

    #[test]
    fn push_left_test() {
        let mut t = BinTree::new();
        t.push_left(4);
        t.push_left(2);
        t.push_left(8);
        t.push_left(1);
        assert_eq!(t.to_string(),"((((1) <= 8) <= 2) <= 4)");
        assert_eq!(t.to_vec(),vec![1,8,2,4]);
        t.extend_left(vec![18,6,3,5,11]);
        assert_eq!(t.to_string(),"(((((((((11) <= 5) <= 3) <= 6) <= 18) <= 1) <= 8) <= 2) <= 4)");
        assert_eq!(t.to_vec(),vec![11,5,3,6,18,1,8,2,4]);
    }

    #[test]
    fn pop_test() {
        let mut t = test_tree();
        assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(1));
        assert_eq!(t.to_string(),"((3) <= 2 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(2));
        assert_eq!(t.to_string(),"(3 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(3));
        assert_eq!(t.to_string(),"(4 => (5 => (6)))");
        assert_eq!(t.pop(),Some(4));
        assert_eq!(t.to_string(),"(5 => (6))");
        assert_eq!(t.pop(),Some(5));
        assert_eq!(t.to_string(),"(6)");
        assert_eq!(t.pop(),Some(6));
        assert_eq!(t.to_string(),"()");
        assert_eq!(t.pop(),None);
    }

    #[cfg(feature = "rand")]
    #[test]
    fn remove_sorted_test() {
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        let mut t = BinTree::new();
        let mut v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
        v.shuffle(&mut thread_rng());
        t.extend_sorted(v);
        let mut v = t.to_vec();
        v.shuffle(&mut thread_rng());
        for i in v {
            assert_eq!(t.remove_sorted(&i),true);
        }
        assert_eq!(t.to_string(),"()");
    }

    #[cfg(feature = "rand")]
    #[test]
    fn remove_sorted_test_10000() {
        for _ in 0..10000 {
            remove_sorted_test();
        }
    }

    #[test]
    fn collect_test() {
        let v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
        let t = v.into_iter().collect::<BinTree<_>>();
        assert_eq!(t.to_string(),
            "((((1 => (2 => (2))) <= 3 => ((3 => (3 => (3))) <= 5 => (5))) <= 6 => (((6) <= 7) <= 8 => ((8 => ((9) <= 10)) <= 11))) <= 18)");
    }

}