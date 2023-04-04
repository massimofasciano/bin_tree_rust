use crate::BinTree;

enum BinTreeIntoIterStackItem<T> {
    Item(T),
    Tree(BinTree<T>),
}

pub struct BinTreeIntoIter<T> {
    stack: Vec<BinTreeIntoIterStackItem<T>>
}

impl<T> IntoIterator for BinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        BinTreeIntoIter { stack: vec![BinTreeIntoIterStackItem::Tree(self)] }
    }
}

impl<T> Iterator for BinTreeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIntoIterStackItem::Item(item)) => Some(item),
            Some(BinTreeIntoIterStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIntoIterStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIntoIterStackItem::Tree(*right));
                self.stack.push(BinTreeIntoIterStackItem::Item(item));
                self.stack.push(BinTreeIntoIterStackItem::Tree(*left));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIterStackItem<'a,T> {
    Item(&'a T),
    Tree(&'a BinTree<T>),
}

pub struct BinTreeIter<'a, T> {
    stack: Vec<BinTreeIterStackItem<'a,T>>
}

impl<'a, T> BinTree<T> {
    pub fn iter(&'a self) -> BinTreeIter<'a, T> {
        BinTreeIter { stack: vec![BinTreeIterStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for BinTreeIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIterStackItem::Item(item)) => Some(item),
            Some(BinTreeIterStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIterStackItem::Tree(right.as_ref()));
                self.stack.push(BinTreeIterStackItem::Item(item));
                self.stack.push(BinTreeIterStackItem::Tree(left.as_ref()));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIterMutStackItem<'a,T> {
    Item(&'a mut T),
    Tree(&'a mut BinTree<T>),
}

pub struct BinTreeIterMut<'a, T> {
    stack: Vec<BinTreeIterMutStackItem<'a,T>>
}

impl<'a, T> BinTree<T> {
    pub fn iter_mut(&'a mut self) -> BinTreeIterMut<'a, T> {
        BinTreeIterMut { stack: vec![BinTreeIterMutStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for BinTreeIterMut<'a,T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIterMutStackItem::Item(item)) => Some(item),
            Some(BinTreeIterMutStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterMutStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIterMutStackItem::Tree(right.as_mut()));
                self.stack.push(BinTreeIterMutStackItem::Item(item));
                self.stack.push(BinTreeIterMutStackItem::Tree(left.as_mut()));
                self.next()
            }
        }
    }
}

