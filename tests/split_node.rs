use bin_tree::{tree, leaf};

#[test]
fn test_macros() {
    let mut bt = tree(4,leaf(1),tree(7,leaf(6),()));
    assert_eq!(bt.to_string(),"((1) <= 4 => ((6) <= 7))");

    let (value,left,right) = bt.node().expect("tree should not be empty");
    assert_eq!(value,&4);
    assert_eq!(left.to_string(),"(1)");
    assert_eq!(right.to_string(),"((6) <= 7)");

    let (value,left,right) = bt.node_mut().expect("tree should not be empty");
    *value += 1;
    *left = tree(2,leaf(1),leaf(3));
    let p = right.pop_sorted(true);
    assert_eq!(value,&5);
    assert_eq!(p,Some(7));
    assert_eq!(left.to_string(),"((1) <= 2 => (3))");
    assert_eq!(right.to_string(),"(6)");

    let (value,left,right) = bt.into_node().expect("tree should not be empty");
    assert_eq!(value,5);
    assert_eq!(left.to_string(),"((1) <= 2 => (3))");
    assert_eq!(right.to_string(),"(6)");
}