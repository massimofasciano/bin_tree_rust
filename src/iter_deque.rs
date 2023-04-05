use std::collections::VecDeque;

use crate::BinTree;

/// tree traversal methods: depth-first (3 orders), breadth-first
enum BinTreeTraversal {
    DepthFirst(DepthFirstOrder),
    BreadthFirst,
}

/// 3 orders for depth-first: in order, pre-order, post-order
enum DepthFirstOrder {
    InOrder,
    PreOrder,
    PostOrder,
}

use BinTreeTraversal::*;
use DepthFirstOrder::*;

//
// into_iter
//

/// deque items
enum BinTreeIntoIterDataItem<T> {
    Item(T),
    Tree(BinTree<T>),
}

/// iterator struct using a deque
pub struct BinTreeIntoIter<T> {
    data: VecDeque<BinTreeIntoIterDataItem<T>>,
    traversal: BinTreeTraversal,
}

impl<T> IntoIterator for BinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    /// default is depth-first in-order
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
    /// depth-first in-order iterator
    pub fn into_iter_dfs_in(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(InOrder))
    }
    /// depth-first pre-order iterator
    pub fn into_iter_dfs_pre(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(PreOrder))
    }
    /// depth-first post-order iterator
    pub fn into_iter_dfs_post(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(DepthFirst(PostOrder))
    }
    /// breadth-first iterator
    pub fn into_iter_bfs(self) -> BinTreeIntoIter<T> {
        self.into_iter_traversal(BreadthFirst)
    }
}

impl<T> Iterator for BinTreeIntoIter<T> {
    type Item = T;

    /// a deque is used to push and pop from both ends according to the specified traversal behavior
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

//
// iter
//

/// deque items
enum BinTreeIterDataItem<'a,T> {
    Item(&'a T),
    Tree(&'a BinTree<T>),
}

/// iterator struct using a deque
pub struct BinTreeIter<'a, T> {
    data: VecDeque<BinTreeIterDataItem<'a,T>>,
    traversal: BinTreeTraversal,
}

impl<'a, T> IntoIterator for &'a BinTree<T> {
    type IntoIter = BinTreeIter<'a, T>;
    type Item = &'a T;

    /// into_iter for ref is iter
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> BinTree<T> {
    // default iterator is depth-first in-order
    pub fn iter(&'a self) -> BinTreeIter<'a, T> {
        self.iter_dfs_in()
    }
    fn iter_traversal(&'a self, traversal : BinTreeTraversal) -> BinTreeIter<'a, T> {
        BinTreeIter { 
            data: VecDeque::from(vec![BinTreeIterDataItem::Tree(self)]),
            traversal,
        }
    }
    /// depth-first in-order iterator
    pub fn iter_dfs_in(&'a self) -> BinTreeIter<'a, T> {
        self.iter_traversal(DepthFirst(InOrder))
    }
    /// depth-first pre-order iterator
    pub fn iter_dfs_pre(&'a self) -> BinTreeIter<'a, T> {
        self.iter_traversal(DepthFirst(PreOrder))
    }
    /// depth-first post-order iterator
    pub fn iter_dfs_post(&'a self) -> BinTreeIter<'a, T> {
        self.iter_traversal(DepthFirst(PostOrder))
    }
    /// breadth-first iterator
    pub fn iter_bfs(&'a self) -> BinTreeIter<'a, T> {
        self.iter_traversal(BreadthFirst)
    }
}

impl<'a,T> Iterator for BinTreeIter<'a,T> {
    type Item = &'a T;

    /// a deque is used to push and pop from both ends according to the specified traversal behavior
    fn next(&mut self) -> Option<Self::Item> {
        let pop = match self.traversal {
            DepthFirst(_) => self.data.pop_back(),
            BreadthFirst => self.data.pop_front(),
        };
        match pop {
            None => None,
            Some(BinTreeIterDataItem::Item(item)) => Some(item),
            Some(BinTreeIterDataItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterDataItem::Tree(BinTree::Branch(item, left, right))) => {
                match self.traversal {
                    DepthFirst(InOrder) => {
                        self.data.push_back(BinTreeIterDataItem::Tree(right.as_ref()));
                        self.data.push_back(BinTreeIterDataItem::Item(item));
                        self.data.push_back(BinTreeIterDataItem::Tree(left.as_ref()));
                        self.next()
                    },
                    DepthFirst(PreOrder) => {
                        self.data.push_back(BinTreeIterDataItem::Tree(right.as_ref()));
                        self.data.push_back(BinTreeIterDataItem::Tree(left.as_ref()));
                        self.data.push_back(BinTreeIterDataItem::Item(item));
                        self.next()
                    },
                    DepthFirst(PostOrder) => {
                        self.data.push_back(BinTreeIterDataItem::Item(item));
                        self.data.push_back(BinTreeIterDataItem::Tree(right.as_ref()));
                        self.data.push_back(BinTreeIterDataItem::Tree(left.as_ref()));
                        self.next()

                    },
                    BreadthFirst => {
                        self.data.push_back(BinTreeIterDataItem::Item(item));
                        self.data.push_back(BinTreeIterDataItem::Tree(left.as_ref()));
                        self.data.push_back(BinTreeIterDataItem::Tree(right.as_ref()));
                        self.next()
                    },
                }
            }
        }
    }
}

//
// iter_mut
//

/// deque items
enum BinTreeIterMutDataItem<'a,T> {
    Item(&'a mut T),
    Tree(&'a mut BinTree<T>),
}

/// iterator struct using a deque
pub struct BinTreeIterMut<'a, T> {
    data: VecDeque<BinTreeIterMutDataItem<'a,T>>,
    traversal: BinTreeTraversal,
}

impl<'a, T> IntoIterator for &'a mut BinTree<T> {
    type IntoIter = BinTreeIterMut<'a, T>;
    type Item = &'a mut T;

    /// into_iter for ref mut is iter_mut
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> BinTree<T> {
    // default iterator is depth-first in-order
    pub fn iter_mut(&'a mut self) -> BinTreeIterMut<'a, T> {
        self.iter_mut_dfs_in()
    }
    fn iter_mut_traversal(&'a mut self, traversal : BinTreeTraversal) -> BinTreeIterMut<'a, T> {
        BinTreeIterMut { 
            data: VecDeque::from(vec![BinTreeIterMutDataItem::Tree(self)]),
            traversal,
        }
    }
    /// depth-first in-order iterator
    pub fn iter_mut_dfs_in(&'a mut self) -> BinTreeIterMut<'a, T> {
        self.iter_mut_traversal(DepthFirst(InOrder))
    }
    /// depth-first pre-order iterator
    pub fn iter_mut_dfs_pre(&'a mut self) -> BinTreeIterMut<'a, T> {
        self.iter_mut_traversal(DepthFirst(PreOrder))
    }
    /// depth-first post-order iterator
    pub fn iter_mut_dfs_post(&'a mut self) -> BinTreeIterMut<'a, T> {
        self.iter_mut_traversal(DepthFirst(PostOrder))
    }
    /// breadth-first iterator
    pub fn iter_mut_bfs(&'a mut self) -> BinTreeIterMut<'a, T> {
        self.iter_mut_traversal(BreadthFirst)
    }
}

impl<'a,T> Iterator for BinTreeIterMut<'a,T> {
    type Item = &'a mut T;

    /// a deque is used to push and pop from both ends according to the specified traversal behavior
    fn next(&mut self) -> Option<Self::Item> {
        let pop = match self.traversal {
            DepthFirst(_) => self.data.pop_back(),
            BreadthFirst => self.data.pop_front(),
        };
        match pop {
            None => None,
            Some(BinTreeIterMutDataItem::Item(item)) => Some(item),
            Some(BinTreeIterMutDataItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterMutDataItem::Tree(BinTree::Branch(item, left, right))) => {
                match self.traversal {
                    DepthFirst(InOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.next()
                    },
                    DepthFirst(PreOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.next()
                    },
                    DepthFirst(PostOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.next()

                    },
                    BreadthFirst => {
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.next()
                    },
                }
            }
        }
    }
}
