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
    /// consumes self and returns the value at the top of the tree
    pub fn into_value(self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.root.unwrap().value)
        }
    }
    /// consumes self and returns the left branch of the tree
    pub fn into_left(self) -> Option<BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            Some(self.root.unwrap().left)
        }
    }
    /// consumes self and returns the right branch of the tree
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

impl<T> From<()> for BinTree<T> {
    /// build an empty tree from the empty type
    fn from(_: ()) -> Self {
        Self::new()
    }
}

/// convenient function to construct a tree from value and branches
pub fn tree<T>(value: T, left: impl Into<BinTree<T>>, right: impl Into<BinTree<T>>) -> BinTree<T> {
    BinTree::new_node(value, left.into(), right.into())
}

/// convenient function to construct a tree leaf from a value
pub fn leaf<T>(value: T) -> BinTree<T> {
    BinTree::new_leaf(value)
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
    }

}