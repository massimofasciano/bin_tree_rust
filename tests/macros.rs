use bintree_iterators::{macros::*, tree, leaf};

#[test]
fn test_macros() {
    let mut bt = tree(4,leaf(1),tree(7,leaf(6),()));
    assert_eq!(bt.to_string(),"((1) <= 4 => ((6) <= 7))");

    let_node_ref!(bt => value, left, right);
    assert_eq!(value,&4);
    assert_eq!(left.to_string(),"(1)");
    assert_eq!(right.to_string(),"((6) <= 7)");

    let_node_ref_mut!(bt => value, left, right);
    *value += 1;
    *left = tree(2,leaf(1),leaf(3));
    let p = right.pop_sorted();
    assert_eq!(value,&5);
    assert_eq!(p,Some(7));
    assert_eq!(left.to_string(),"((1) <= 2 => (3))");
    assert_eq!(right.to_string(),"(6)");

    let_node_move!(bt => value, left, right);
    assert_eq!(value,5);
    assert_eq!(left.to_string(),"((1) <= 2 => (3))");
    assert_eq!(right.to_string(),"(6)");

    let mut bt = tree(4,leaf(1),tree(7,leaf(6),()));
    assert_eq!(bt.to_string(),"((1) <= 4 => ((6) <= 7))");

    let_node_ref_mut!(bt => value, _left, right);
    let value = take_value_replace_tree!(&mut bt, value, right);
    assert_eq!(value, 4);
    assert_eq!(bt.to_string(),"((6) <= 7)");
}