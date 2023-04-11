use bin_tree::{FormattedBinTree, FormattedBinTreeType, tree, leaf, OrderedSetBinTree, BinTreeMap};

#[test]
fn demo() {
    let mut t = 
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
        )));
        
    assert_eq!(format!("{}",t),
        "(((3) <= 2) <= 1 => (4 => ((6) <= 5)))");

    // wrapper class to provide different display methods
    let ft = FormattedBinTree::new(&t,FormattedBinTreeType::PrettyIndent("--"));
    assert_eq!(format!("{}", ft),"\
    ------@\n\
    ----5\n\
    --------@\n\
    ------6\n\
    --------@\n\
    --4\n\
    ----@\n\
    1\n\
    ----@\n\
    --2\n\
    ------@\n\
    ----3\n\
    ------@\n\
    ");

    // false (because the tree is not sorted and this method depends on it for the search)
    // by wrapping in an ordered set or map, we avoid these issues (see below)
    assert_eq!(t.contains_sorted(&2),false);
    
    assert_eq!(t.contains(&2),true);
    
    // true (happy accident: tree is partially sorted to the right)
    // by wrapping in an ordered set or map, we avoid these issues (see below)
    assert_eq!(t.contains_sorted(&5),true);
    
    // binary tree iter_mut
    for i in t.iter_mut() {
        print!("{}",i);
        if *i % 3 == 0 {
            *i /= 3;
        }
    }
    assert_eq!(t.to_vec(),vec![1,2,1,4,2,5]);
    
    // binary tree ordered set
    let v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
    let mut t = v.into_iter().collect::<OrderedSetBinTree<_>>();
    assert_eq!(t.to_string(),"[1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 18]");
    assert_eq!(t.inner().to_string(),
        "((((1 => (2)) <= 3 => (5)) <= 6 => ((7) <= 8 => (((9) <= 10) <= 11))) <= 18)");
    
    assert_eq!(t.contains(&2),true);
    assert_eq!(t.contains(&10),true);
    assert_eq!(t.contains(&4),false);
    
    assert_eq!(t.remove(&7),true);
    assert_eq!(format!("{}",t),"[1, 2, 3, 5, 6, 8, 9, 10, 11, 18]");
    assert_eq!(t.to_tree_string(),
        "((((1 => (2)) <= 3 => (5)) <= 6 => (8 => (((9) <= 10) <= 11))) <= 18)");

    // binary tree map
    let mut t : BinTreeMap<char, usize> = BinTreeMap::new();
    t.insert('a', 782);
    t.insert('b', 1782);
    t.insert('c', 500);
    assert_eq!(t.to_string(),"[('a', 782), ('b', 1782), ('c', 500)]");
    assert_eq!(t.contains_key(&'b'),true);
    assert_eq!(t.contains_key(&'z'),false);
    assert_eq!(t.get(&'a'),Some(&782));
    assert_eq!(t.swap(&'a', &'c'),true);
    assert_eq!(t.to_string(),"[('a', 500), ('b', 1782), ('c', 782)]");
    *t.get_mut(&'b').unwrap() += 8;
    assert_eq!(t.to_string(),"[('a', 500), ('b', 1790), ('c', 782)]");
    assert_eq!(t.remove(&'b'),Some(1790));
    assert_eq!(t.to_string(),"[('a', 500), ('c', 782)]");

}
