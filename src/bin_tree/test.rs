use crate::{BinTree, tree, leaf, BinTreeError};

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
fn eq_test() {
    let bt = 
        BinTree::new_node(1,
            BinTree::new_node(2,
                BinTree::new_leaf(3),
                BinTree::new(),
            ),
            BinTree::new_node(4,
                BinTree::new(),
                BinTree::new_node(5, 
                    BinTree::new_leaf(6), 
                    BinTree::new()
                )
            )
        );
    let t = test_tree();
    assert_eq!(t,bt);
}

#[test]
fn basic_access_test() {
    let mut tree : BinTree<i32> = Default::default();
    assert_eq!(tree.is_empty(),true);
    tree = BinTree::new_leaf(10);
    assert_eq!(tree.is_leaf(),true);
    assert_eq!(tree.value().unwrap(),&10);
    *tree.left_mut().unwrap() = BinTree::new_leaf(20);
    assert_eq!(tree.is_leaf(),false);
    assert_eq!(tree.is_branch(),true);
    assert_eq!(tree.to_string(),"((20) <= 10)");
    *tree.left_mut().unwrap() = BinTree::new();
    *tree.right_mut().unwrap() = BinTree::new_leaf(30);
    assert_eq!(tree.is_empty(),false);
    assert_eq!(tree.is_leaf(),false);
    assert_eq!(tree.is_branch(),true);
    assert_eq!(tree.to_string(),"(10 => (30))");
    *tree.get_mut(&30).unwrap() = 40;
    assert_eq!(tree.to_string(),"(10 => (40))");
    let subtree = tree.get_tree_mut(&40).unwrap();
    *subtree = BinTree::new_leaf(50);
    assert_eq!(tree.to_string(),"(10 => (50))");
    *tree.get_mut_sorted(&50).unwrap() = 60;
    assert_eq!(tree.to_string(),"(10 => (60))");
    let subtree = tree.get_tree_mut_sorted(&60).unwrap();
    *subtree = BinTree::new_leaf(70);
    assert_eq!(tree.to_string(),"(10 => (70))");
}

#[test]
fn test_init_height() {
    let t = test_tree();

    assert_eq!(format!("{:?}",t),"\
        BinTree { root: Some(BinTreeNode { value: 1, \
            left: BinTree { root: Some(BinTreeNode { value: 2, \
                left: BinTree { root: Some(BinTreeNode { value: 3, \
                    left: BinTree { root: None, height: 0 }, \
                    right: BinTree { root: None, height: 0 } }), \
                    height: 1 }, \
                right: BinTree { root: None, height: 0 } }), \
                height: 2 }, \
            right: BinTree { root: Some(BinTreeNode { value: 4, \
                left: BinTree { root: None, height: 0 }, \
                right: BinTree { root: Some(BinTreeNode { value: 5, \
                    left: BinTree { root: Some(BinTreeNode { value: 6, \
                        left: BinTree { root: None, height: 0 }, \
                        right: BinTree { root: None, height: 0 } }), \
                        height: 1 }, \
                    right: BinTree { root: None, height: 0 } }), \
                    height: 2 } }), \
                height: 3 } }), \
            height: 4 }");
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

#[test]
fn take_replace_swap_test() {
    let mut t = test_tree();

    let value1 = std::mem::take(t.get_mut(&5).unwrap());
    assert_eq!(value1,5);
    assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (4 => ((6) <= 0)))");

    let value2 = std::mem::replace(t.get_mut(&4).unwrap(),value1);
    assert_eq!(value2,4);
    assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (5 => ((6) <= 0)))");

    assert_eq!(t.swap(&3, &1).is_ok(),true);
    assert_eq!(t.swap(&5, &6).is_ok(),true);
    assert_eq!(t.swap(&0, &6).is_ok(),true);
    assert_eq!(t.swap(&1, &1),Err(BinTreeError::SwapSame));
    assert_eq!(t.swap(&1, &10),Err(BinTreeError::SwapNotFound2));
    assert_eq!(t.swap(&10, &1),Err(BinTreeError::SwapNotFound1));
    assert_eq!(t.to_string(),"(((1) <= 2) <= 3 => (0 => ((5) <= 6)))");
    *t.get_mut(&0).unwrap() = 4;
    assert_eq!(t.to_vec(),vec![1,2,3,4,5,6]);
}
#[test]
fn push_sorted_test() {
    let mut t = BinTree::new();
    t.insert(4);
    t.insert(2);
    t.insert(8);
    t.insert(1);
    t.insert(2);
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
    assert_eq!(t.insert_unique(4),true);
    assert_eq!(t.insert_unique(2),true);
    assert_eq!(t.insert_unique(8),true);
    assert_eq!(t.insert_unique(1),true);
    assert_eq!(t.insert_unique(2),false);
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
    macro_rules! insert_cmp {
        ($t:ident, $s:expr) => {
            $t.insert_to_key_cmp($s,&|i|i,cmp,true,true)    
        };
    }
    insert_cmp!(t,"hello there");
    insert_cmp!(t,"hello there!");
    insert_cmp!(t,"hello my name is Rusty");
    // "hello world!" replaces "hello there!" because same length...
    assert_eq!(insert_cmp!(t,"hello world!"),Some("hello there!")); 
    assert_eq!(insert_cmp!(t,"hello"),None);
    assert_eq!(t.to_vec(),vec!["hello", "hello there", "hello world!", "hello my name is Rusty"]);
    for s in &t {
        assert_eq!(t.get_sorted_to_key_cmp(s, &|x|x, cmp),Some(s));
    }
    assert_eq!(t.remove_sorted_to_key_cmp(&"hello world!", &|x|x, cmp, true),Some("hello world!"));
    assert_eq!(t.to_vec(),vec!["hello", "hello there", "hello my name is Rusty"]);
    let s = t.get_mut_sorted_to_key_cmp(&"hello", &|x|x, cmp).unwrap();
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

    t.insert_unique(1);
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
        t.insert_to_key_cmp(c,&|c|c,&char::partial_cmp,false, true);
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
        t.insert_to_key_cmp(c,&|c|c,&char::partial_cmp,true, true);
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

#[test]
fn pop_sorted_height_test() {
    let s = "This is a very long string for my TEST!";
    let mut t;

    t = BinTree::new();
    t.extend_sorted_unique(s.chars());
    assert_eq!(t.len(),20);
    assert_eq!(String::from_iter(t.to_vec().iter())," !ESTaefghilmnorstvy");
    assert_eq!(t.height(),5);

    assert_eq!(t.recalculate_heights(),false);
    assert_eq!(t.pop_sorted(),Some('h'));
    assert_eq!(t.recalculate_heights(),false);
    assert_eq!(t.pop_sorted(),Some('i'));
    assert_eq!(t.recalculate_heights(),false);
    assert_eq!(t.pop_sorted(),Some('l'));
    assert_eq!(t.recalculate_heights(),false);
    assert_eq!(t.pop_sorted(),Some('m'));
    assert_eq!(t.recalculate_heights(),false);

    for _ in 0..t.len() {
        assert_eq!(t.pop_sorted().is_some(),true);
        assert_eq!(t.recalculate_heights(),false);
    }

    assert_eq!(t.is_empty(),true);        
}
#[test]
fn remove_sorted_height_test() {
    let s = "This is a very long string for my TEST!";
    let mut t;

    t = BinTree::new();
    t.extend_sorted_unique(s.chars());
    assert_eq!(t.len(),20);
    assert_eq!(String::from_iter(t.to_vec().iter())," !ESTaefghilmnorstvy");
    assert_eq!(t.height(),5);

    assert_eq!(t.recalculate_heights(),false);
    assert_eq!(t.remove_sorted(&'r'),Some('r'));
    assert_eq!(t.recalculate_heights(),false);

    for c in t.to_vec() {
        assert_eq!(t.remove_sorted(&c),Some(c));
        assert_eq!(t.recalculate_heights(),false);
    }

    assert_eq!(t.is_empty(),true);        
}
