use crate::{BinTree, let_node_ref};

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
    /// number of elements in the tree
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// rebalance a balanced binary tree
    pub fn rebalance(&mut self) {
        if !self.is_empty() {
            if self.balance() < -1 && self.left().unwrap().balance() == -1 {
                self.rotate_right();
            } else if self.balance() > 1 && self.right().unwrap().balance() == 1 {
                self.rotate_left();
            } else if self.balance() < -1 && self.left().unwrap().balance() == 1 {
                self.rotate_left_right();
            } else if self.balance() > 1 && self.right().unwrap().balance() == -1 {
                self.rotate_right_left();
            }
        } 
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_right(&mut self) {
        if !self.is_empty() {
            let mut l = std::mem::take(self.left_mut().unwrap());
            let l_r = std::mem::take(l.right_mut().unwrap());
            _ = std::mem::replace(self.left_mut().unwrap(), l_r);
            let mut n = std::mem::take(self);
            n.height = std::cmp::max(n.left().unwrap().height(),n.right().unwrap().height()) + 1;
            _ = std::mem::replace(l.right_mut().unwrap(), n);
            _ = std::mem::replace(self, l);
            self.update_height();
        }
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_left(&mut self) {
        if !self.is_empty() {
            let mut r = std::mem::take(self.right_mut().unwrap());
            let r_l = std::mem::take(r.left_mut().unwrap());
            _ = std::mem::replace(self.right_mut().unwrap(), r_l);
            let mut n = std::mem::take(self);
            n.height = std::cmp::max(n.left().unwrap().height(),n.right().unwrap().height()) + 1;
            _ = std::mem::replace(r.left_mut().unwrap(), n);
            _ = std::mem::replace(self, r);
            self.update_height();
        }
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_left_right(&mut self) {
        if !self.is_empty() {
            self.left_mut().unwrap().rotate_left();
            self.rotate_right();
            self.update_height();
        }
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_right_left(&mut self) {
        if !self.is_empty() {
            self.right_mut().unwrap().rotate_right();
            self.rotate_left();
            self.update_height();
        }
    }

    /// returns the mutable tree node containing the minimum value item
    /// assumes that the tree is sorted
    pub(crate) fn min_tree_mut(&mut self) -> Option<&mut BinTree<Item>> {
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
