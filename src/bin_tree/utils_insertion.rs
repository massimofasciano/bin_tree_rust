use crate::{BinTree, let_node_ref_mut};

impl<Item> BinTree<Item> {
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
}