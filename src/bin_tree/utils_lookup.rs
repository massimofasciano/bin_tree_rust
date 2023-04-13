use crate::{BinTree, let_node_ref_mut, let_node_ref};

impl<Item> BinTree<Item> {
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
