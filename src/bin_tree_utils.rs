use crate::{BinTree, let_node_ref_mut, let_node_ref};

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
        if self.is_empty() {
            write!(f,"()")
        } else {
            let_node_ref!(self => value, left, right);
            write!(f,"(")?;
            if !left.is_empty() {
                left.write_line(f)?;
                write!(f," <= ")?;
            }
            write!(f,"{:?}",value)?;
            if !right.is_empty() {
                write!(f," => ")?;
                right.write_line(f)?;
            }
            write!(f,")")
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
        if self.is_empty() {
            write!(f,"{}{}\n",tab.repeat(indent),"@")
        } else {
            let_node_ref!(self => value, left, right);
            right.pretty_write_indent(f, tab, indent+1)?;
            write!(f,"{}{:?}\n",tab.repeat(indent),value)?;
            left.pretty_write_indent(f, tab, indent+1)
        }
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
        if self.is_empty() {
            *self = Self::new_leaf(new_item)
        } else {
            let_node_ref_mut!(self => item, left, right);
            if new_item < *item {
                left.push_sorted(new_item);
            } else {
                right.push_sorted(new_item);
            }
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
        if self.is_empty() {
            *self = Self::new_leaf(new_item)
        } else {
            let_node_ref_mut!(self => item, left, right);
            if new_item < *item {
                left.push_sorted_unique(new_item);
            } else if new_item > *item {
                right.push_sorted_unique(new_item);
            }
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
        if let Some(right) = self.right_mut() {
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
        if let Some(left) = self.left_mut() {
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
    /// take value of tree root and replace the root with other tree, return the taken value
    pub fn take_and_replace_with(&mut self, tree: &mut BinTree<Item>) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let value = std::mem::replace(self.value_mut().unwrap(), unsafe { 
                std::mem::MaybeUninit::uninit().assume_init() 
            });
            let tree = std::mem::take(tree);
            *self = tree;
            Some(value)
        }
    }
    /// pop the top item from the tree
    pub fn pop(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let mut p;
            p = self.left_mut().unwrap().pop();
            if p.is_none() {
                p = self.right_mut().unwrap().pop();
            }
            Some(match p {
                None => {
                    self.take_and_replace_with(&mut Self::new()).unwrap()
                },
                Some(value) => {
                    std::mem::replace(self.value_mut().unwrap(), value)
                },
            })

        }
    }
    /// returns the mutable tree node containing the minimum value item
    /// assumes that the tree is sorted
    pub fn min_tree_mut(&mut self) -> Option<&mut BinTree<Item>> where Item : PartialOrd {
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
    /// returns the mutable tree node containing the maximum value item
    /// assumes that the tree is sorted
    pub fn max_tree_mut(&mut self) -> Option<&mut BinTree<Item>> where Item : PartialOrd {
        if self.is_leaf() {
            Some(self)
        } else if self.is_branch() {
            if self.right().unwrap().is_empty() {
                // no right path
                Some(self)
            } else {
                // max from right path
                self.right_mut().unwrap().max_tree_mut()

            }
        } else {
            None
        }
    }
    /// pop the top value from a sorted tree and preserves order
    pub fn pop_sorted(&mut self) -> Option<Item> where Item : PartialOrd {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => item, left, right);
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
        }
    }
    /// try to remove value from a sorted tree and preserve order
    pub fn remove_sorted(&mut self, target_value : &Item) -> bool where Item : PartialOrd {
        if self.is_empty() {
            false
        } else {
            let_node_ref_mut!(self => value, left, right);
            if *target_value < *value {
                left.remove_sorted(target_value)
            } else if *target_value > *value {
                right.remove_sorted(target_value)
            } else {
                self.pop_sorted();
                true
            }
        }
    }
    /// find a value in a sorted tree
    pub fn contains_sorted(&self, target_value : &Item) -> bool where Item : PartialOrd {
        if self.is_empty() {
            false
        } else {
            let_node_ref!(self => value, left, right);
            if *target_value < *value {
                left.contains_sorted(target_value)
            } else if *target_value > *value {
                right.contains_sorted(target_value)
            } else {
                true
            }
        }
    }
    /// find a value in a tree (no ordering assumed)
    pub fn contains(&self, target_value : &Item) -> bool where Item : PartialEq {
        if self.is_empty() {
            false
        } else {
            let_node_ref!(self => value, left, right);
            target_value == value || 
            left.contains(target_value) || 
            right.contains(target_value)
        }
    }
    /// find a value in a tree and return mutable ref (no ordering assumed)
    pub fn get_mut(&mut self, target_value : &Item) -> Option<&mut Item> where Item : PartialEq {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if target_value == value {
                Some(value)
            } else if let Some(left_get) = left.get_mut(target_value) {
                Some(left_get)
            } else {
                right.get_mut(target_value)
            }
        }
    }
    /// find a value in a tree and return mutable ref to the subtree (no ordering assumed)
    pub fn get_tree_mut(&mut self, target_value : &Item) -> Option<&mut BinTree<Item>> where Item : PartialEq {
        if self.is_empty() {
            None
        } else if target_value == self.value().unwrap() {
            Some(self)
        } else {
            let_node_ref_mut!(self => _value, left, right);
            let mut tree = left.get_tree_mut(target_value);
            if tree.is_none() {
                tree = right.get_tree_mut(target_value);
            }
            tree
        }
    }
    /// find a value in a sorted tree and return mutable ref
    pub fn get_sorted_mut(&mut self, target_value : &Item) -> Option<&mut Item> where Item : PartialOrd {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if *target_value < *value {
                left.get_sorted_mut(target_value)
            } else if *target_value > *value {
                right.get_sorted_mut(target_value)
            } else {
                Some(value)
            }
        }
    }
    /// find a value in a sorted tree and return mutable ref
    pub fn get_tree_sorted_mut(&mut self, value : &Item) -> Option<&mut BinTree<Item>> where Item : PartialOrd {
        if self.is_empty() {
            None
        } else if *value < *self.value().unwrap() {
            self.left_mut().unwrap().get_tree_sorted_mut(value)
        } else if *value > *self.value().unwrap() {
            self.right_mut().unwrap().get_tree_sorted_mut(value)
        } else {
            Some(self)
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
