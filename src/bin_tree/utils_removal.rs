use crate::{BinTree, let_node_ref_mut};

impl<Item> BinTree<Item> {

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
