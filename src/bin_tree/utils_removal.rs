use crate::BinTree;

impl<Item> BinTree<Item> {

    /// try to remove value from a tree
    /// heights are not adjusted
    pub fn remove(&mut self, target_value : &Item) -> Option<Item> where Item : PartialEq {
        if self.is_empty() {
            None
        } else {
            let (value,left,right) = self.node_mut().expect("tree should not be empty");
            // let_node_ref_mut!(self => value, left, right);
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
    pub fn remove_sorted(&mut self, target_value : &Item) -> Option<Item> where Item : PartialOrd {
        self.remove_sorted_to_key_cmp(target_value, |x|x, Item::partial_cmp, true)
    }

    /// try to remove sorted tree and preserve order
    /// uses key and compare functions
    /// optional rebalancing
    /// heights are preserved
    pub fn remove_sorted_to_key_cmp<FtoKey,Fcmp,Key>(&mut self, target_key : &Key, 
        to_key: FtoKey, cmp : Fcmp, rebalance : bool) -> Option<Item> where 
        Fcmp : Fn(&Key, &Key) -> Option<std::cmp::Ordering>,
        FtoKey : Fn(&Item) -> &Key,
    {
        if self.is_empty() {
            None
        } else {
            let (value,left,right) = self.node_mut().expect("tree should not be empty");
            // let_node_ref_mut!(self => value, left, right);
            let result = match cmp(target_key, to_key(value)) {
                Some(std::cmp::Ordering::Less) => left.remove_sorted_to_key_cmp(target_key,to_key,cmp,rebalance),
                Some(std::cmp::Ordering::Greater) => right.remove_sorted_to_key_cmp(target_key,to_key,cmp,rebalance),
                _ => self.pop_sorted(rebalance),
            };
            if result.is_some() {
                self.update_height();
                if rebalance { self.rebalance(); }
            }
            result
        }
    }

    /// pop the top node from a sorted tree and preserves order
    /// heights are adjusted
    /// rebalancing is optional
    pub fn pop_tree_sorted(&mut self, rebalance : bool) -> Option<BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            if self.left().unwrap().is_empty() && self.right().unwrap().is_empty() {
                Some(std::mem::take(self))
            } else if self.right().unwrap().is_empty() {
                let left = std::mem::take(self.left_mut().unwrap());
                Some(std::mem::replace(self, left))
            } else if self.left().unwrap().is_empty() {
                let right = std::mem::take(self.right_mut().unwrap());
                Some(std::mem::replace(self, right))
            } else {
                let mut new_self = self.right_mut().unwrap().detach_left_min_tree_sorted(rebalance).unwrap();
                std::mem::swap(self.left_mut().unwrap(), new_self.left_mut().unwrap());
                std::mem::swap(self.right_mut().unwrap(), new_self.right_mut().unwrap());
                std::mem::swap(self, &mut new_self);
                self.update_height();
                Some(new_self)
            }
        }
    }

    /// detaches the mutable tree node containing the minimum value item
    /// assumes that the tree is sorted
    /// the tree is adjusted on the way up, including heights and optional rebalancing
    pub fn detach_left_min_tree_sorted(&mut self, rebalance : bool) -> Option<BinTree<Item>> {
        if self.is_leaf() {
            Some(std::mem::take(self))
        } else if self.is_branch() {
            if self.left().unwrap().is_empty() {
                // no left path
                let right = std::mem::take(self.right_mut().unwrap());
                Some(std::mem::replace(self, right))
            } else {
                // min from left path
                let result = self.left_mut().unwrap().detach_left_min_tree_sorted(rebalance);
                self.update_height();
                if rebalance { self.rebalance() };
                result
            }
        } else {
            None
        }
    }

    /// pop the top node from the tree
    /// heights are not adjusted
    pub fn pop_tree(&mut self) -> Option<BinTree<Item>> {
        if self.is_empty() {
            None
        } else {
            let mut p;
            p = self.left_mut().unwrap().pop_tree();
            if p.is_none() {
                p = self.right_mut().unwrap().pop_tree();
            }
            Some(match p {
                None => {
                    std::mem::take(self)
                },
                Some(mut new_self) => {
                    std::mem::swap(self.left_mut().unwrap(), new_self.left_mut().unwrap());
                    std::mem::swap(self.right_mut().unwrap(), new_self.right_mut().unwrap());
                    std::mem::swap(self, &mut new_self);
                    new_self
                },
            })
        }
    }

    /// pop the top item from the tree
    /// heights are not adjusted
    pub fn pop(&mut self) -> Option<Item> {
        let pop_tree = self.pop_tree();
        if let Some(pop_tree) = pop_tree {
            pop_tree.into_value()
        } else {
            None
        }
    }
    /// pop the top value from a sorted tree and preserves order
    /// heights are adjusted
    /// rebalancing is optional
    pub fn pop_sorted(&mut self, rebalance : bool) -> Option<Item> {
        let pop_tree = self.pop_tree_sorted(rebalance);
        if let Some(pop_tree) = pop_tree {
            pop_tree.into_value()
        } else {
            None
        }
    }

    /// pop from the left of tree
    pub fn pop_left(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let (_,left,right) = self.node_mut().expect("tree should not be empty");
            // let_node_ref_mut!(self => _value, left, right);
            if left.is_empty() && right.is_empty() {
                std::mem::take(self).into_value()
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
            let (_,left,right) = self.node_mut().expect("tree should not be empty");
            // let_node_ref_mut!(self => _value, left, right);
            if left.is_empty() && right.is_empty() {
                std::mem::take(self).into_value()
            } else if right.is_empty() {
                self.pop()
            } else {
                right.pop_right()
            }
        }
    }

}
