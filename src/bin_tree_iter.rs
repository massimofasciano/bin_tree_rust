use std::{collections::VecDeque};

use crate::{BinTree, let_node_ref, let_node_move, let_node_ref_mut};

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

/// Value or Tree
enum IterIntoData<Item> {
    Value(Item),
    Tree(BinTree<Item>)
}
/// iterator struct using a deque
pub struct BinTreeIntoIter<T> {
    data: VecDeque<IterIntoData<T>>,
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
            data: VecDeque::from(vec![IterIntoData::Tree(self)]),
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
        use IterIntoData::*;
        match pop {
            None => None, // no more work
            Some(Value(item)) => Some(item),
            Some(Tree(tree)) => {
                if tree.is_empty() {
                    self.next()
                } else {
                    let_node_move!(tree => value, left, right);
                    match self.traversal {
                        DepthFirst(InOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.next()
                        },
                        DepthFirst(PreOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Value(value));
                            self.next()
                        },
                        DepthFirst(PostOrder) => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.next()

                        },
                        BreadthFirst => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Tree(right));
                            self.next()
                        },
                    }
                }
            }
        }
    }
}

//
// iter
//

/// Value or Tree
enum IterData<'a, Item> {
    Value(&'a Item),
    Tree(&'a BinTree<Item>)
}

/// iterator struct using a deque
pub struct BinTreeIter<'a, T> {
    data: VecDeque<IterData<'a,T>>,
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
            data: VecDeque::from(vec![IterData::Tree(self)]),
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
        use IterData::*;
        match pop {
            None => None, // no more work
            Some(Value(item)) => Some(item),
            Some(Tree(tree)) => {
                if tree.is_empty() {
                    self.next()
                } else {
                    let_node_ref!(tree => value, left, right);
                    match self.traversal {
                        DepthFirst(InOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.next()
                        },
                        DepthFirst(PreOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Value(value));
                            self.next()
                        },
                        DepthFirst(PostOrder) => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.next()

                        },
                        BreadthFirst => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Tree(right));
                            self.next()
                        },
                    }
                }
            }
        }
    }
}

//
// iter_mut
//

/// Value or Tree
enum IterMutData<'a, Item> {
    Value(&'a mut Item),
    Tree(&'a mut BinTree<Item>)
}

/// iterator struct using a deque
pub struct BinTreeIterMut<'a, T> {
    data: VecDeque<IterMutData<'a,T>>,
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
            data: VecDeque::from(vec![IterMutData::Tree(self)]),
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
        use IterMutData::*;
        match pop {
            None => None, // no more work
            Some(Value(item)) => Some(item),
            Some(Tree(tree)) => {
                if tree.is_empty() {
                    self.next()
                } else {
                    let_node_ref_mut!(tree => value, left, right);
                    match self.traversal {
                        DepthFirst(InOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.next()
                        },
                        DepthFirst(PreOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Value(value));
                            self.next()
                        },
                        DepthFirst(PostOrder) => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.next()

                        },
                        BreadthFirst => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Tree(right));
                            self.next()
                        },
                    }
                }
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
    fn iter_mut_test() {
        let mut t = test_tree();
        t.iter_mut().for_each(|i| {
            if *i % 2 == 1 { *i += 10 }
        });
        assert_eq!(t.to_string(),"(((13) <= 2) <= 11 => (4 => ((6) <= 15)))");
    }

    #[test]
    fn into_iter_order_test() {
        let t = test_tree();
        assert_eq!(t.into_iter().collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_in().collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_pre().collect::<Vec<_>>(),vec![1, 2, 3, 4, 5, 6]);
        let t = test_tree();
        assert_eq!(t.into_iter_dfs_post().collect::<Vec<_>>(),vec![3, 2, 6, 5, 4, 1]);
        let t = test_tree();
        assert_eq!(t.into_iter_bfs().collect::<Vec<_>>(),vec![1, 2, 4, 3, 5, 6]);
    }

    #[test]
    fn iter_order_test() {
        let t = test_tree();
        assert_eq!(t.iter().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        assert_eq!(t.iter_dfs_in().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 1, 4, 6, 5]);
        assert_eq!(t.iter_dfs_pre().map(|i|i.clone()).collect::<Vec<_>>(),vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(t.iter_dfs_post().map(|i|i.clone()).collect::<Vec<_>>(),vec![3, 2, 6, 5, 4, 1]);
        assert_eq!(t.iter_bfs().map(|i|*i).collect::<Vec<_>>(),vec![1, 2, 4, 3, 5, 6]);
    }

    #[test]
    fn iter_mut_order_test() {
        let mut t = test_tree();
        let mut i = 0;
        t.iter_mut().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![4, 4, 4, 8, 11, 11]);
        t.iter_mut_dfs_in().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![11, 12, 13, 18, 22, 23]);
        t.iter_mut_dfs_pre().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![26, 26, 26, 34, 40, 40]);
        t.iter_mut_dfs_post().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![45, 46, 50, 57, 61, 62]);
        t.iter_mut_bfs().for_each(|e| { i += 1; *e += i; });
        assert_eq!(t.to_vec(),vec![73, 72, 75, 84, 91, 91]);
    }
}