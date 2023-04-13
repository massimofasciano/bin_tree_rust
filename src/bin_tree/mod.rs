use crate::let_node_ref_mut;

/// various tools for the binary tree
pub mod utils;
pub use self::utils::*;

/// tools for the binary tree that use bits of unsafe code
pub mod utils_unsafe;
pub use self::utils_unsafe::*;

/// tools for the binary tree (for insertions)
pub mod utils_insertion;
pub use self::utils_insertion::*;

/// tools for the binary tree (for removals)
pub mod utils_removal;
pub use self::utils_removal::*;

/// tools for the binary tree (for lookup/get)
pub mod utils_lookup;
pub use self::utils_lookup::*;

/// iterators (owned,ref,mut) over a binary tree with 4 traversal methods
pub mod iter;
pub use self::iter::*;

/// a formatted tree container shows how to implement custom display behavior
pub mod formatted;
pub use self::formatted::*;

/// a general purpose binary tree
#[derive(Debug,Clone,PartialEq)]
pub struct BinTree<Item> {
    pub root: Option<Box<BinTreeNode<Item>>>,
    pub height: isize, // this field is only updated when representing balanced trees
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
        let height = std::cmp::max(left.height(),right.height()) + 1;
        Self { 
            root : Some(Box::new(BinTreeNode{value, left, right})), 
            height,
        }
    }
    /// creates a leaf
    pub fn new_leaf(item : Item) -> Self {
        Self::new_node(item, Self::new(), Self::new())
    }
    /// creates an empty tree
    pub fn new() -> Self {
        Self { root : None, height : 0 }
    }
    /// height of tree
    pub fn height(&self) -> isize {
        self.height
    }
    /// height of tree
    pub fn update_height(&mut self) -> isize {
        if !self.is_empty() {
            self.height = std::cmp::max(self.left().unwrap().height(),self.right().unwrap().height()) + 1;
            self.height
        } else {
            0
        }
    }
    /// balance of tree
    pub fn balance(&self) -> isize {
        if self.is_empty() {
            0
        } else {
            self.left().unwrap().height()-
            self.right().unwrap().height()
        }
    }
    /// is the tree balanced ?
    pub fn is_balanced(&self) -> bool {
        if self.is_empty() {
            true
        } else {
            let b = self.balance();
            b >= -1 && 
            b <= 1 && 
            self.left().unwrap().is_balanced() &&
            self.right().unwrap().is_balanced()
        }
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
    /// recalculate the height fields in the tree
    /// return true if any height changed ?
    pub fn recalculate_heights(&mut self) -> bool {
        self.recalculate_heights_rec(true, true, false).1
    }
    /// recalculate the height fields in the tree
    /// return (height of tree, any height changed ?)
    /// recursion to left and/or right is optional (for special optimized cases)
    /// optional rebalancing
    pub fn recalculate_heights_rec(&mut self, rec_left : bool, rec_right : bool, rebalance : bool) -> (isize, bool) {
        let mut changed = false;
        if self.is_empty() {
            self.height = 0;
        } else {
            let_node_ref_mut!(self => _value, left, right);
            let mut height_left = left.height();
            let mut height_right = right.height();
            let mut changed_left = false;
            let mut changed_right = false;
            if rec_left {
                (height_left, changed_left) = left.recalculate_heights_rec(rec_left,rec_right,rebalance);
            }
            if rec_right {
                (height_right, changed_right) = right.recalculate_heights_rec(rec_left,rec_right,rebalance);
            }
            let height_rec = std::cmp::max(height_left,height_right) + 1;
            if changed_left || changed_right {
                changed = true;
            }
            if self.height != height_rec {
                changed = true;
                self.height = height_rec;
            }
            if rebalance { self.rebalance() }
        }
        (self.height, changed)
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

#[cfg(test)]
mod test;
