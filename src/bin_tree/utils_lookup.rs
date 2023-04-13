use crate::{BinTree, let_node_ref_mut, let_node_ref};

impl<Item> BinTree<Item> {

        /// find a value in a sorted tree with key and compare functions and return ref
        pub fn get_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&self, target_key : &Key,
            to_key: &FtoKey, cmp : &Fcmp) -> Option<&Item> where 
            Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
            FtoKey : Fn(&Item) -> &Key,
        {
            if self.is_empty() {
                None
            } else {
                let_node_ref!(self => value, left, right);
                match cmp(target_key, to_key(value)) {
                    Some(std::cmp::Ordering::Less) => left.get_sorted_to_key_cmp(target_key,to_key,cmp),
                    Some(std::cmp::Ordering::Greater) => right.get_sorted_to_key_cmp(target_key,to_key,cmp),
                    _ => Some(value),
                }
            }
        }
    
        /// find a value in a sorted tree with key and compare functions and return mut ref
        pub fn get_mut_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, target_key : &Key,
            to_key: &FtoKey, cmp : &Fcmp) -> Option<&mut Item> where 
            Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
            FtoKey : Fn(&Item) -> &Key,
        {
            if self.is_empty() {
                None
            } else {
                let_node_ref_mut!(self => value, left, right);
                match cmp(target_key, to_key(value)) {
                    Some(std::cmp::Ordering::Less) => left.get_mut_sorted_to_key_cmp(target_key,to_key,cmp),
                    Some(std::cmp::Ordering::Greater) => right.get_mut_sorted_to_key_cmp(target_key,to_key,cmp),
                    _ => Some(value),
                }
            }
        }

        /// find a value in a sorted tree and return ref
        pub fn get_sorted(&self, target_value : &Item) -> Option<&Item> where Item : PartialOrd {
            self.get_sorted_to_key_cmp(target_value, &|x|x, &Item::partial_cmp)
        }

        /// find a value in a sorted tree and return mutable ref
        pub fn get_mut_sorted(&mut self, target_value : &Item) -> Option<&mut Item> where Item : PartialOrd {
            self.get_mut_sorted_to_key_cmp(target_value, &|x|x, &Item::partial_cmp)
        }

        /// find a value in a sorted tree with key and compare functions and return mut ref
        pub fn get_tree_mut_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, target_key : &Key,
            to_key: &FtoKey, cmp : &Fcmp) -> Option<&mut BinTree<Item>> where 
            Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
            FtoKey : Fn(&Item) -> &Key,
        {
            if self.is_empty() {
                None
            } else {
                match cmp(target_key, to_key(self.value().unwrap())) {
                    Some(std::cmp::Ordering::Less) => self.left_mut().unwrap().get_tree_mut_sorted_to_key_cmp(target_key,to_key,cmp),
                    Some(std::cmp::Ordering::Greater) => self.right_mut().unwrap().get_tree_mut_sorted_to_key_cmp(target_key,to_key,cmp),
                    _ => Some(self),
                }
            }
        }

        /// find a value in a sorted tree and return mutable ref
        pub fn get_tree_mut_sorted(&mut self, value : &Item) -> Option<&mut BinTree<Item>> where Item : PartialOrd {
            self.get_tree_mut_sorted_to_key_cmp(value, &|x|x, &Item::partial_cmp)
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

        /// find a value in a sorted tree
        pub fn contains_sorted(&self, target_value : &Item) -> bool where Item : PartialOrd {
            self.get_sorted(target_value).is_some()
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

}
