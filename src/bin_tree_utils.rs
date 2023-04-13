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
            self.height = std::cmp::max(self.left().unwrap().height(),self.right().unwrap().height()) + 1;
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
            self.height = std::cmp::max(self.left().unwrap().height(),self.right().unwrap().height()) + 1;
        }
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_left_right(&mut self) {
        if !self.is_empty() {
            self.left_mut().unwrap().rotate_left();
            self.rotate_right();
            self.height = std::cmp::max(self.left().unwrap().height(),self.right().unwrap().height()) + 1;
        }
    }
    /// utility function used in rebalancing of a balanced binary tree
    pub fn rotate_right_left(&mut self) {
        if !self.is_empty() {
            self.right_mut().unwrap().rotate_right();
            self.rotate_left();
            self.height = std::cmp::max(self.left().unwrap().height(),self.right().unwrap().height()) + 1;
        }
    }

    /// default push method (uses push_sorted)
    pub fn push(&mut self, new_item : Item) where Item : PartialOrd {
        self.push_sorted(new_item);
    }
    /// push onto a sorted or empty tree and keeps order property (rebalance)
    pub fn push_sorted(&mut self, new_item : Item) where Item : PartialOrd {
        self.push_sorted_maybe_rebalance(new_item, true);
    }
    /// push onto a sorted or empty tree and keeps order property (optional rebalancing)
    pub fn push_sorted_maybe_rebalance(&mut self, new_item : Item, rebalance : bool) where Item : PartialOrd {
        if self.is_empty() {
            *self = Self::new_leaf(new_item)
        } else {
            let_node_ref_mut!(self => item, left, right);
            if new_item < *item {
                left.push_sorted_maybe_rebalance(new_item,rebalance);
            } else {
                right.push_sorted_maybe_rebalance(new_item,rebalance);
            }
            self.height = std::cmp::max(left.height, right.height) + 1;
            if rebalance { self.rebalance(); }
        }
    }

    /// extend a sorted or empty tree and keeps order property (rebalance)
    pub fn extend_sorted<T: IntoIterator<Item = Item>>(&mut self, iter: T) where Item : PartialOrd {
        for elem in iter {
            self.push_sorted(elem);
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
        if self.is_empty() {
            *self = Self::new_leaf(new_item);
            None
        } else {
            let_node_ref_mut!(self => item, left, right);
            let result = match cmp(to_key(&new_item), to_key(item)) {
                Some(std::cmp::Ordering::Less) => left.push_sorted_unique_to_key_cmp(new_item,to_key,cmp,rebalance),
                Some(std::cmp::Ordering::Greater) => right.push_sorted_unique_to_key_cmp(new_item,to_key,cmp,rebalance),
                _ => Some(std::mem::replace(item, new_item)),
            };
            self.height = std::cmp::max(left.height, right.height) + 1;
            if rebalance { self.rebalance(); }
            result
        }
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
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if *target_value < *value {
                left.remove_sorted(target_value)
            } else if *target_value > *value {
                right.remove_sorted(target_value)
            } else {
                self.pop_sorted()
            }
        }
    }
    /// try to remove with key from a sorted tree and preserve order
    pub fn remove_sorted_with_key<F,Key>(&mut self, target_value : &Key, key: &F) -> Option<Item> where
        Key : PartialOrd,
        F : Fn(&Item) -> &Key,
        Item : Default,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if target_value < key(value) {
                left.remove_sorted_with_key(target_value,key)
            } else if target_value > key(value) {
                right.remove_sorted_with_key(target_value,key)
            } else {
                self.pop_sorted_with_key(key)
            }
        }
    }
    /// try to remove with compare function from a sorted tree and preserve order
    pub fn remove_sorted_with_compare<F>(&mut self, target_value : &Item, compare : &F) -> Option<Item> where
        F : Fn(&Item, &Item) -> Option<std::cmp::Ordering>,
        Item : Default,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            match compare(target_value, value) {
                Some(std::cmp::Ordering::Less) => left.remove_sorted_with_compare(target_value,compare),
                Some(std::cmp::Ordering::Greater) => right.remove_sorted_with_compare(target_value,compare),
                _ => self.pop_sorted_with_compare(compare),
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
    pub fn pop_sorted(&mut self) -> Option<Item> where Item : PartialOrd {
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
                min_right.pop_sorted()
            }
        }
    }
    /// pop the top value from a sorted tree and preserves order (using key function)
    pub fn pop_sorted_with_key<F,Key>(&mut self, key: &F) -> Option<Item> where
        Key : PartialOrd,
        F : Fn(&Item) -> &Key,
    {
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
                min_right.pop_sorted_with_key(key)
            }
        }
    }
    /// pop the top value from a sorted tree and preserves order (using compare function)
    pub fn pop_sorted_with_compare<F>(&mut self, compare: &F) -> Option<Item> where
        F : Fn(&Item, &Item) -> Option<std::cmp::Ordering>,
    {
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
                min_right.pop_sorted_with_compare(compare)
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
    fn push_sorted_test() {
        let mut t = BinTree::new();
        t.push_sorted(4);
        t.push_sorted(2);
        t.push_sorted(8);
        t.push_sorted(1);
        t.push_sorted(2);
        assert_eq!(t.to_string(),"(((1) <= 2 => (2)) <= 4 => (8))");
        assert_eq!(t.to_vec(),vec![1,2,2,4,8]);
        t.extend_sorted(vec![18,6,3,8,5,11]);
        assert_eq!(t.to_string(),"(((1) <= 2 => (2 => (3))) <= 4 => (((5) <= 6) <= 8 => ((8) <= 11 => (18))))");
        assert_eq!(t.height(),4);
        assert_eq!(t.to_vec(),vec![1,2,2,3,4,5,6,8,8,11,18]);
    }

    #[test]
    fn push_sorted_unique_test() {
        let mut t = BinTree::new();
        assert_eq!(t.push_sorted_unique(4),true);
        assert_eq!(t.push_sorted_unique(2),true);
        assert_eq!(t.push_sorted_unique(8),true);
        assert_eq!(t.push_sorted_unique(1),true);
        assert_eq!(t.push_sorted_unique(2),false);
        assert_eq!(t.to_string(),"(((1) <= 2) <= 4 => (8))");
        assert_eq!(t.to_vec(),vec![1,2,4,8]);
        assert_eq!(t.extend_sorted_unique(vec![18,6,3,8,5,11]),5);
        assert_eq!(t.to_string(),"(((1) <= 2 => (3)) <= 4 => (((5) <= 6) <= 8 => ((11) <= 18)))");
        assert_eq!(t.to_vec(),vec![1,2,3,4,5,6,8,11,18]);
    }

    #[test]
    fn push_right_test() {
        let mut t = BinTree::new();
        t.push_right(4);
        t.push_right(2);
        t.push_right(8);
        t.push_right(1);
        assert_eq!(t.to_string(),"(4 => (2 => (8 => (1))))");
        assert_eq!(t.to_vec(),vec![4,2,8,1]);
        t.extend_right(vec![18,6,3,5,11]);
        assert_eq!(t.to_string(),"(4 => (2 => (8 => (1 => (18 => (6 => (3 => (5 => (11)))))))))");
        assert_eq!(t.to_vec(),vec![4,2,8,1,18,6,3,5,11]);
    }

    #[test]
    fn push_left_test() {
        let mut t = BinTree::new();
        t.push_left(4);
        t.push_left(2);
        t.push_left(8);
        t.push_left(1);
        assert_eq!(t.to_string(),"((((1) <= 8) <= 2) <= 4)");
        assert_eq!(t.to_vec(),vec![1,8,2,4]);
        t.extend_left(vec![18,6,3,5,11]);
        assert_eq!(t.to_string(),"(((((((((11) <= 5) <= 3) <= 6) <= 18) <= 1) <= 8) <= 2) <= 4)");
        assert_eq!(t.to_vec(),vec![11,5,3,6,18,1,8,2,4]);
    }

    #[test]
    fn pop_test() {
        let mut t = test_tree();
        assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(1));
        assert_eq!(t.to_string(),"((3) <= 2 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(2));
        assert_eq!(t.to_string(),"(3 => (4 => ((6) <= 5)))");
        assert_eq!(t.pop(),Some(3));
        assert_eq!(t.to_string(),"(4 => (5 => (6)))");
        assert_eq!(t.pop(),Some(4));
        assert_eq!(t.to_string(),"(5 => (6))");
        assert_eq!(t.pop(),Some(5));
        assert_eq!(t.to_string(),"(6)");
        assert_eq!(t.pop(),Some(6));
        assert_eq!(t.to_string(),"()");
        assert_eq!(t.pop(),None);
    }

    // to enable randomized order, test with 
    // cargo test --features rand to enable (or --all-features)
    #[test]
    fn remove_sorted_test() {
        #[cfg(feature = "rand")]
        use rand::thread_rng;
        #[cfg(feature = "rand")]
        use rand::seq::SliceRandom;

        let mut t = BinTree::new();
        #[allow(unused_mut)]
        let mut v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
        #[cfg(feature = "rand")]
        v.shuffle(&mut thread_rng());
        t.extend_sorted(v);
        #[allow(unused_mut)]
        let mut v = t.to_vec();
        #[cfg(feature = "rand")]
        v.shuffle(&mut thread_rng());
        for i in v {
            assert_eq!(t.remove_sorted(&i),Some(i));
        }
        assert_eq!(t.to_string(),"()");
    }

    #[cfg(feature = "rand")]
    #[test]
    fn remove_sorted_test_10000() {
        for _ in 0..10000 {
            remove_sorted_test();
        }
    }

    #[test]
    fn collect_test() {
        let v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
        let t = v.into_iter().collect::<BinTree<_>>();
        assert_eq!(t.to_string(),
            "((((1) <= 2 => (2)) <= 3 => (((3) <= 3 => (3)) <= 5 => (5))) <= 6 => (((6) <= 7 => (8)) <= 8 => (((9) <= 10) <= 11 => (18))))");
        assert_eq!(t.height(),5);
    }

    #[test]
    fn push_pop_test() {
        let mut t = BinTree::new();
        for i in 1..10 {
            t.push_left(i)
        }
        assert_eq!(t.to_vec(),(1..10).rev().collect::<Vec<_>>());
        for i in (1..10).rev() {
            assert_eq!(t.pop_left().unwrap(),i);
        }

        let mut t = BinTree::new();
        for i in 1..10 {
            t.push_right(i)
        }
        assert_eq!(t.to_vec(),(1..10).collect::<Vec<_>>());
        for i in (1..10).rev() {
            assert_eq!(t.pop_right().unwrap(),i);
        }

        let mut t = BinTree::new();
        for i in 1..10 {
            t.push_left(i)
        }
        assert_eq!(t.to_vec(),(1..10).rev().collect::<Vec<_>>());
        for i in 1..10 {
            assert_eq!(t.pop_right().unwrap(),i);
        }

        let mut t = BinTree::new();
        for i in 1..10 {
            t.push_right(i)
        }
        assert_eq!(t.to_vec(),(1..10).collect::<Vec<_>>());
        for i in 1..10 {
            assert_eq!(t.pop_left().unwrap(),i);
        }
    }

    #[test]
    fn ordered_compare_test() {
        let mut t = BinTree::new();
        let cmp = &|s1: &&str,s2: &&str| s1.len().partial_cmp(&s2.len());
        t.push_sorted_unique_cmp("hello there", cmp);
        t.push_sorted_unique_cmp("hello there!", cmp);
        t.push_sorted_unique_cmp("hello my name is Rusty", cmp);
        // "hello world!" replaces "hello there!" because same length...
        assert_eq!(t.push_sorted_unique_cmp("hello world!", cmp),Some("hello there!")); 
        assert_eq!(t.push_sorted_unique_cmp("hello", cmp),None);
        assert_eq!(t.to_vec(),vec!["hello", "hello there", "hello world!", "hello my name is Rusty"]);
        for s in &t {
            assert_eq!(t.get_sorted_with_compare(s, cmp),Some(s));
        }
        assert_eq!(t.remove_sorted_with_compare(&"hello world!", cmp),Some("hello world!"));
        assert_eq!(t.to_vec(),vec!["hello", "hello there", "hello my name is Rusty"]);
        let s = t.get_sorted_mut_with_compare(&"hello", cmp).unwrap();
        assert_eq!(s,&"hello");
        *s = "do you like the borrow checker?";
        // The tree is not sorted anymore because we used a mut reference to change a value
        // without using a normal insertion that preserves order.
        assert_eq!(t.to_vec(),vec!["do you like the borrow checker?", "hello there", "hello my name is Rusty"]);
    }

    #[test]
    fn test_height() {
        let mut t = BinTree::new();
        assert_eq!(t.height(),0);

        t.push_sorted_unique(1);
        assert_eq!(t.height(),1);

        t.extend_sorted_unique(vec![5,3,7,38,9,20,4,5,6,17,24,3,1,12,3,24,5,6,7,2,4,6,16]);
        assert_eq!(t.height(),5);
        assert_eq!(format!("{:?}",t),"\
            BinTree { root: Some(BinTreeNode { value: 7, \
                left: BinTree { root: Some(BinTreeNode { value: 3, \
                    left: BinTree { root: Some(BinTreeNode { value: 1, \
                        left: BinTree { root: None, height: 0 }, \
                        right: BinTree { root: Some(BinTreeNode { value: 2, \
                            left: BinTree { root: None, height: 0 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 1 } }), \
                        height: 2 }, \
                    right: BinTree { root: Some(BinTreeNode { value: 5, \
                        left: BinTree { root: Some(BinTreeNode { value: 4, \
                            left: BinTree { root: None, height: 0 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 1 }, \
                        right: BinTree { root: Some(BinTreeNode { value: 6, \
                            left: BinTree { root: None, height: 0 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 1 } }), \
                        height: 2 } }), \
                    height: 3 }, \
                right: BinTree { root: Some(BinTreeNode { value: 20, \
                    left: BinTree { root: Some(BinTreeNode { value: 12, \
                        left: BinTree { root: Some(BinTreeNode { value: 9, \
                            left: BinTree { root: None, height: 0 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 1 }, \
                        right: BinTree { root: Some(BinTreeNode { value: 17, \
                            left: BinTree { root: Some(BinTreeNode { value: 16, \
                                left: BinTree { root: None, height: 0 }, \
                                right: BinTree { root: None, height: 0 } }), \
                                height: 1 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 2 } }), \
                        height: 3 }, \
                    right: BinTree { root: Some(BinTreeNode { value: 38, \
                        left: BinTree { root: Some(BinTreeNode { value: 24, \
                            left: BinTree { root: None, height: 0 }, \
                            right: BinTree { root: None, height: 0 } }), \
                            height: 1 }, \
                        right: BinTree { root: None, height: 0 } }), \
                        height: 2 } }), \
                    height: 4 } }), \
                height: 5 }\
        ");
        assert_eq!(t.to_vec(),vec![1, 2, 3, 4, 5, 6, 7, 9, 12, 16, 17, 20, 24, 38]);
    }

    #[test]
    fn test_rebalance_off_on() {
        let s = "This is a very long string for my TEST!";
        let mut t;

        t = BinTree::new();
        for c in s.chars() {
            t.push_sorted_unique_to_key_cmp(c,&|c|c,&char::partial_cmp,false);
        }
        assert_eq!(t.height(),9);
        assert_eq!(t.to_string(),
            "(\
                (' ' => (('!') <= 'E' => ('S'))) \
            <= 'T' => \
                (('a' => ('e' => (('f') <= 'g'))) <= 'h' => ('i' => \
                    ((('l' => ((('m') <= 'n') <= 'o')) <= 'r') <= 's' => (('t') <= 'v' => ('y')))))\
            )"
        );

        t = BinTree::new();
        for c in s.chars() {
            t.push_sorted_unique_to_key_cmp(c,&|c|c,&char::partial_cmp,true);
        }
        assert_eq!(t.height(),5);
        assert_eq!(t.to_string(),
            "(\
                (((' ' => ('!')) <= 'E' => (('S') <= 'T' => ('a'))) <= 'e' => (('f') <= 'g')) \
            <= 'h' => \
                (((('i') <= 'l' => ('m')) <= 'n' => ('o' => ('r'))) <= 's' => (('t') <= 'v' => ('y')))\
            )"
        );
    }
}
