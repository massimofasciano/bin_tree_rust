/// a general purpose binary tree
#[derive(Debug,Clone,PartialEq)]
#[repr(transparent)]
pub struct BinTree<Item> {
    pub root: Option<Box<BinTreeNode<Item>>>
}

/// a general purpose binary tree node
#[derive(Debug,Clone,PartialEq)]
pub struct BinTreeNode<Item> {
    pub value : Item,
    pub left : BinTree<Item>,
    pub right : BinTree<Item>,
}

impl<Item> BinTree<Item> {
    /// creates a branch
    pub fn new_node(value : Item, left: BinTree<Item>, right: BinTree<Item>) -> Self {
        Self { root : Some(Box::new(BinTreeNode{value, left, right}))}
    }
    /// creates a leaf
    pub fn new_leaf(item : Item) -> Self {
        Self::new_node(item, Self::new(), Self::new())
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
    /// returns a ref to the value at the top of the tree
    pub fn value(&self) -> Option<&Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self.root.as_deref().unwrap().value)
        }
    }
    /// returns the left branch of the tree
    pub fn left(&self) -> Option<&BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(&self.root.as_deref().unwrap().left)
        }
    }
    /// returns the right branch of the tree
    pub fn right(&self) -> Option<&BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(&self.root.as_deref().unwrap().right)
        }
    }
    /// returns a mutable ref to the 3 elements of the node at the top of the tree (if not empty)
    pub fn node_mut(&mut self) -> Option<(&mut Item,&mut BinTree<Item>,&mut BinTree<Item>)> {
        if self.is_empty() {
            None
        } else {
            let tree = self.root.as_deref_mut().unwrap();
            Some((&mut tree.value,&mut tree.left,&mut tree.right))
        }
    }
    /// returns a mutable ref to the value at the top of the tree
    pub fn value_mut(&mut self) -> Option<&mut Item> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.root.as_deref_mut().unwrap().value)
        }
    }
    /// returns a mutable ref to the left branch of the tree
    pub fn left_mut(&mut self) -> Option<&mut BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.root.as_deref_mut().unwrap().left)
        }
    }
    /// returns a mutable ref to the right branch of the tree
    pub fn right_mut(&mut self) -> Option<&mut BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.root.as_deref_mut().unwrap().right)
        }
    }
    /// takes the value at the top of the tree
    pub fn into_value(self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.root.unwrap().value)
        }
    }
    /// takes the left branch of the tree
    pub fn into_left(self) -> Option<BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(self.root.unwrap().left)
        }
    }
    /// takes the right branch of the tree
    pub fn into_right(self) -> Option<BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(self.root.unwrap().right)
        }
    }
    /// sets node at root of tree
    pub fn set_node(&mut self, value : Item, left : BinTree<Item>, right : BinTree<Item>) {
        if self.is_empty() {
            *self = Self::new_node(value,left,right);
        } else {
            let tree = self.root.as_deref_mut().unwrap();
            tree.value = value;
            tree.left = left;
            tree.right = right;
        }
    }
    /// sets value at root of tree
    pub fn set_value(&mut self, value : Item) {
        if self.is_empty() {
            *self = Self::new_leaf(value);
        } else {
            let tree = self.root.as_deref_mut().unwrap();
            tree.value = value;
        }
    }
    /// sets left child at root of tree
    pub fn set_left(&mut self, left : BinTree<Item>) -> bool {
        if self.is_empty() {
            false
        } else {
            let tree = self.root.as_deref_mut().unwrap();
            tree.left = left;
            true
        }
    }
    /// sets right child at root of tree
    pub fn set_right(&mut self, right : BinTree<Item>) -> bool {
        if self.is_empty() {
            false
        } else {
            let tree = self.root.as_deref_mut().unwrap();
            tree.right = right;
            true
        }
    }
    /// sets tree root
    pub fn set(&mut self, tree : BinTree<Item>) {
        *self = tree;
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
            BinTree::new_node(1,
                BinTree::new_node(2,
                    BinTree::new_leaf(3),
                    BinTree::new(),
                ),
                BinTree::new_node(4,
                    BinTree::new(),
                    BinTree::new_node(5, 
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

    #[test]
    fn basic_access_test() {
        let mut tree : BinTree<i32> = Default::default();
        assert_eq!(tree.is_empty(),true);
        tree = BinTree::new_leaf(10);
        assert_eq!(tree.is_leaf(),true);
        assert_eq!(tree.value().unwrap(),&10);
        *tree.left_mut().unwrap() = BinTree::new_leaf(20);
        assert_eq!(tree.is_leaf(),false);
        assert_eq!(tree.is_branch(),true);
        assert_eq!(tree.to_string(),"((20) <= 10)");
        *tree.left_mut().unwrap() = BinTree::new();
        *tree.right_mut().unwrap() = BinTree::new_leaf(30);
        assert_eq!(tree.is_empty(),false);
        assert_eq!(tree.is_leaf(),false);
        assert_eq!(tree.is_branch(),true);
        assert_eq!(tree.to_string(),"(10 => (30))");
        *tree.get_mut(&30).unwrap() = 40;
        assert_eq!(tree.to_string(),"(10 => (40))");
        let subtree = tree.get_tree_mut(&40).unwrap();
        *subtree = BinTree::new_leaf(50);
        assert_eq!(tree.to_string(),"(10 => (50))");
        *tree.get_sorted_mut(&50).unwrap() = 60;
        assert_eq!(tree.to_string(),"(10 => (60))");
        let subtree = tree.get_tree_sorted_mut(&60).unwrap();
        *subtree = BinTree::new_leaf(70);
        assert_eq!(tree.to_string(),"(10 => (70))");
        let subtree = tree.get_tree_sorted_mut(&70).unwrap();
        let old = subtree.take_and_replace_with(&mut BinTree::new()).unwrap();
        assert_eq!(old,70);
        assert_eq!(tree.to_string(),"(10)");
    }
}