use std::collections::VecDeque;

use crate::BinTree;

enum BinTreeTraversal {
    DepthFirst(DepthFirstOrder),
    BreadthFirst,
}

enum DepthFirstOrder {
    InOrder,
    PreOrder,
    PostOrder,
}

use BinTreeTraversal::*;
use DepthFirstOrder::*;

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIntoIterDataItem<T> {
    Item(T),
    Tree(BinTree<T>),
}

pub struct BinTreeIntoIter<T> {
    data: VecDeque<BinTreeIntoIterDataItem<T>>,
    traversal: BinTreeTraversal,
}

impl<T> IntoIterator for BinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter_dfs_in()
    }
}

impl<T> BinTree<T> {
    fn into_iter_traversal(self, traversal : BinTreeTraversal) -> BinTreeIntoIter<T> {
        BinTreeIntoIter { 
            data: VecDeque::from(vec![BinTreeIntoIterDataItem::Tree(self)]),
            traversal,
        }
    }
    pub fn into_iter_dfs_in(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(InOrder))
    }
    pub fn into_iter_dfs_pre(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(PreOrder))
    }
    pub fn into_iter_dfs_post(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(PostOrder))
    }
    pub fn into_iter_bfs(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(BreadthFirst)
    }
}

impl<T> Iterator for BinTreeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = match self.traversal {
            DepthFirst(_) => self.data.pop_back(),
            BreadthFirst => self.data.pop_front(),
        };
        match pop {
            None => None,
            Some(BinTreeIntoIterDataItem::Item(item)) => Some(item),
            Some(BinTreeIntoIterDataItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIntoIterDataItem::Tree(BinTree::Branch(item, left, right))) => {
                match self.traversal {
                    DepthFirst(InOrder) => {
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*right));
                        self.data.push_back(BinTreeIntoIterDataItem::Item(item));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*left));
                        self.next()
                    },
                    DepthFirst(PreOrder) => {
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*right));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*left));
                        self.data.push_back(BinTreeIntoIterDataItem::Item(item));
                        self.next()
                    },
                    DepthFirst(PostOrder) => {
                        self.data.push_back(BinTreeIntoIterDataItem::Item(item));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*right));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*left));
                        self.next()

                    },
                    BreadthFirst => {
                        self.data.push_back(BinTreeIntoIterDataItem::Item(item));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*left));
                        self.data.push_back(BinTreeIntoIterDataItem::Tree(*right));
                        self.next()
                    },
                }
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

