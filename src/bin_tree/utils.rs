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

    /// default push method (uses push_sorted)
    pub fn push(&mut self, new_item : Item) where Item : PartialOrd {
        self.push_sorted(new_item);
    }
    /// push onto a sorted or empty tree and keeps order property (rebalance)
    pub fn push_sorted(&mut self, new_item : Item) where Item : PartialOrd {
        // self.push_sorted_maybe_rebalance(new_item, true);
        self.push_sorted_to_key_cmp(new_item, &|x|x, &Item::partial_cmp, true ,false);
    }
    /// extend a sorted or empty tree and keeps order property (rebalance)
    pub fn extend_sorted<T: IntoIterator<Item = Item>>(&mut self, iter: T) where Item : PartialOrd {
        for elem in iter {
            self.push_sorted(elem);
        }
    }

    /// push onto a sorted or empty tree and keeps both properties
    /// unicity (no duplicates) optional
    /// use a function to compare keys and a function to get key from item
    /// returns the replaced item when there is a duplicate (based on compare function)
    /// optional rebalancing
    pub fn push_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, new_item : Item, 
        to_key: &FtoKey, cmp : &Fcmp, rebalance : bool, unique : bool) -> Option<Item> where 
        Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
        FtoKey : Fn(&Item) -> &Key,
    {
        if self.is_empty() {
            *self = Self::new_leaf(new_item);
            None
        } else {
            let mut height_adj = true;
            let_node_ref_mut!(self => item, left, right);
            let result = match cmp(to_key(&new_item), to_key(item)) {
                Some(std::cmp::Ordering::Less) => left.push_sorted_to_key_cmp(new_item,to_key,cmp,rebalance,unique),
                Some(std::cmp::Ordering::Greater) => right.push_sorted_to_key_cmp(new_item,to_key,cmp,rebalance,unique),
                _ => if unique {
                        height_adj = false;
                        Some(std::mem::replace(item, new_item))
                    } else {
                        right.push_sorted_to_key_cmp(new_item,to_key,cmp,rebalance,unique)
                    }
            };
            if height_adj {
                self.height = std::cmp::max(left.height, right.height) + 1;
                if rebalance { self.rebalance(); }
            }
            result
        }
    }
    /// push onto a sorted or empty tree with no duplicates and keeps both properties
    /// use a function to compare keys and a function to get key from item
    /// returns the replaced item when there is a duplicate (based on compare function)
    /// optional rebalancing
    pub fn push_sorted_unique_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, new_item : Item, 
        to_key: &FtoKey, cmp : &Fcmp, rebalance : bool) -> Option<Item> where 
        Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
        FtoKey : Fn(&Item) -> &Key,
    {
        self.push_sorted_to_key_cmp(new_item, to_key, cmp, rebalance, true)
    }
    /// push onto a sorted or empty tree with no duplicates and keeps both properties (rebalance)
    /// returns bool indicating if a new item was added
    pub fn push_sorted_unique(&mut self, new_item : Item) -> bool where Item : PartialOrd {
        self.push_sorted_unique_to_key_cmp(new_item,&|i|i,&Item::partial_cmp,true).is_none()
    }
    /// push onto a sorted or empty tree with no duplicates and keeps both properties (rebalance)
    /// use a function to compare
    /// returns the replaced item when there is a duplicate (based on compare function)
    pub fn push_sorted_unique_cmp<Fcmp>(&mut self, new_item : Item, cmp : &Fcmp) -> Option<Item> where 
        Fcmp : Fn(&Item, &Item) -> Option<std::cmp::Ordering>
    {
        self.push_sorted_unique_to_key_cmp(new_item,&|i|i,cmp,true)
    }
    /// push onto a sorted or empty tree with no duplicates and keeps both properties (rebalance)
    /// use a function to compare based on keys
    /// return the old item when overwriting
    pub fn push_sorted_unique_to_key<FtoKey,Key>(&mut self, new_item : Item, to_key : &FtoKey) -> Option<Item> where 
        Key : PartialOrd,
        FtoKey : Fn(&Item) -> &Key,
    {
        self.push_sorted_unique_to_key_cmp(new_item,to_key,&Key::partial_cmp,true)
    }

    /// extend a sorted or empty tree with no duplicates and keeps both properties (rebalance)
    pub fn extend_sorted_unique<T: IntoIterator<Item = Item>>(&mut self, iter: T) -> usize where Item : PartialOrd {
        let mut count = 0;
        for elem in iter {
            if self.push_sorted_unique(elem) {
                count += 1;
            }
        }
        count
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



    /// returns the mutable tree node containing the minimum value item
    /// assumes that the tree is sorted
    fn min_tree_mut(&mut self) -> Option<&mut BinTree<Item>> {
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
    /// try to remove value from a tree
    /// heights are not adjusted
    pub fn remove(&mut self, target_value : &Item) -> Option<Item> where Item : PartialEq + Default {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if *target_value == *value {
                self.pop()
            } else {
                let mut r = left.remove(target_value);
                if r.is_none() {
                    r = right.remove(target_value);
                }
                r
            }
        }
    }
    /// try to remove value from a sorted tree and preserve order
    pub fn remove_sorted(&mut self, target_value : &Item) -> Option<Item> where Item : PartialOrd + Default {
        self.remove_sorted_to_key_cmp(target_value, &|x|x, &Item::partial_cmp, true)
    }
    /// try to remove with key from a sorted tree and preserve order
    pub fn remove_sorted_with_key<FtoKey,Key>(&mut self, target_key : &Key, to_key: &FtoKey) -> Option<Item> where
        Key : PartialOrd,
        FtoKey : Fn(&Item) -> &Key,
        Item : Default,
    {
        self.remove_sorted_to_key_cmp(target_key, to_key, &Key::partial_cmp, true)
    }
    /// try to remove with compare function from a sorted tree and preserve order
    pub fn remove_sorted_cmp<F>(&mut self, target_value : &Item, cmp : &F) -> Option<Item> where
        F : Fn(&Item, &Item) -> Option<std::cmp::Ordering>,
        Item : Default,
    {
        self.remove_sorted_to_key_cmp(target_value, &|x|x, cmp, true)
    }
    /// try to remove sorted tree and preserve order
    /// uses key and compare functions
    /// optional rebalancing
    /// heights are preserved
    pub fn remove_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, target_key : &Key, 
        to_key: &FtoKey, cmp : &Fcmp, rebalance : bool) -> Option<Item> where 
        Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
        FtoKey : Fn(&Item) -> &Key,
        Item : Default,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            let result = match cmp(target_key, to_key(value)) {
                Some(std::cmp::Ordering::Less) => left.remove_sorted_to_key_cmp(target_key,to_key,cmp,rebalance),
                Some(std::cmp::Ordering::Greater) => right.remove_sorted_to_key_cmp(target_key,to_key,cmp,rebalance),
                _ => self.pop_sorted(),
            };
            self.update_height();
            if rebalance { self.rebalance(); }
            result
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
    /// find a value in a sorted tree with a key function and return ref
    pub fn get_sorted_with_key<F,Key>(&self, target_value : &Key, key : &F) -> Option<&Item> where
        Key : PartialOrd,
        F : Fn(&Item) -> &Key
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref!(self => value, left, right);
            if target_value < key(value) {
                left.get_sorted_with_key(target_value,key)
            } else if target_value > key(value) {
                right.get_sorted_with_key(target_value,key)
            } else {
                Some(value)
            }
        }
    }
    /// find a value in a sorted tree with a key function and return mut ref
    pub fn get_sorted_mut_with_key<F,Key>(&mut self, target_value : &Key, key : &F) -> Option<&mut Item> where
        Key : PartialOrd,
        F : Fn(&Item) -> &Key
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if target_value < key(value) {
                left.get_sorted_mut_with_key(target_value,key)
            } else if target_value > key(value) {
                right.get_sorted_mut_with_key(target_value,key)
            } else {
                Some(value)
            }
        }
    }
    /// find a value in a sorted tree with a compare function and return ref
    pub fn get_sorted_with_compare<F>(&self, target_value : &Item, compare : &F) -> Option<&Item> where
        F : Fn(&Item, &Item) -> Option<std::cmp::Ordering>,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref!(self => value, left, right);
            match compare(target_value, value) {
                Some(std::cmp::Ordering::Less) => left.get_sorted_with_compare(target_value,compare),
                Some(std::cmp::Ordering::Greater) => right.get_sorted_with_compare(target_value,compare),
                _ => Some(value),
            }
        }
    }
    /// find a value in a sorted tree with a compare function and return mut ref
    pub fn get_sorted_mut_with_compare<F>(&mut self, target_value : &Item, compare : &F) -> Option<&mut Item> where
        F : Fn(&Item, &Item) -> Option<std::cmp::Ordering>,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            match compare(target_value, value) {
                Some(std::cmp::Ordering::Less) => left.get_sorted_mut_with_compare(target_value,compare),
                Some(std::cmp::Ordering::Greater) => right.get_sorted_mut_with_compare(target_value,compare),
                _ => Some(value),
            }
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

/// This macro should only used in the specific context of the pop functions in this crate
/// and is not meant as a general purpose tool. It is used to avoid code repetition.
macro_rules! take_value_replace_tree {
    ($dest_tree:expr , $value:expr , $source_tree:expr) => {
        {
            let value_taken = std::mem::take($value);
            let source_tree_taken = std::mem::take($source_tree);
            *$dest_tree = source_tree_taken;
            value_taken
        }
    };
}

impl<Item: Default> BinTree<Item> {
    /// pop the top item from the tree
    /// heights are not adjusted
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
                    take_value_replace_tree!(self,self.value_mut().unwrap(),&mut Self::new())
                },
                Some(value) => {
                    std::mem::replace(self.value_mut().unwrap(), value)
                },
            })

        }
    }
    /// pop the top value from a sorted tree and preserves order
    /// heights are adjusted
    pub fn pop_sorted(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if left.is_empty() && right.is_empty() {
                Some(take_value_replace_tree!(self,value,&mut Self::new()))
            } else if right.is_empty() {
                Some(take_value_replace_tree!(self,value,left))
            } else if left.is_empty() {
                Some(take_value_replace_tree!(self,value,right))
            } else {
                let min_right = right.min_tree_mut().expect("min right should always return some tree");
                let min_right_value = min_right.value_mut().expect("min right should always return some item");
                std::mem::swap(value,min_right_value);
                let result = min_right.pop_sorted();
                // recalc only on the left path of the right subtree
                right.recalculate_heights_rec(true,false);
                self.height = std::cmp::max(left.height, right.height) + 1;
                result
            }
        }
    }

    /// pop from the left of tree
    pub fn pop_left(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if left.is_empty() && right.is_empty() {
                Some(take_value_replace_tree!(self,value,&mut Self::new()))
            } else if left.is_empty() {
                self.pop()
            } else {
                left.pop_left()
            }
        }
    }
    /// pop from the right of tree
    pub fn pop_right(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if left.is_empty() && right.is_empty() {
                Some(take_value_replace_tree!(self,value,&mut Self::new()))
            } else if right.is_empty() {
                self.pop()
            } else {
                right.pop_right()
            }
        }
    }
}

