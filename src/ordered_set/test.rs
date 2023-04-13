use crate::{OrderedSetBinTree, BinTree};

static TEST_STR : &str = "Hello, my name is Joe!";

#[test]
fn test_basic() {
    // let str1 = "(((' ' => ('!')) <= ',') <= 'H' => ((('J') <= 'a') <= \
    //     'e' => (('i') <= 'l' => (('m' => ('n')) <= 'o' => (('s') <= 'y')))))";
    let str1 = 
                "(((' ' \
                    => ('!')) \
            <= ',' => \
                    (('H') \
                <= 'J' => \
                    ('a'))) \
        <= 'e' => \
                ((('i') <= 'l') \
            <= 'm' => \
                    (('n') \
                <= 'o' => \
                        (('s') \
                    <= 'y'))))";
    let mut s = OrderedSetBinTree::new();
    TEST_STR.chars().for_each(|c| s.insert(c));
    assert_eq!(s.to_tree_string(),str1);
    assert_eq!(s.len(), 14);
    let s2 = TEST_STR.chars().collect::<OrderedSetBinTree<_>>();
    assert_eq!(s2.to_tree_string(),str1);
    assert_eq!(s2.len(), 14);
    assert_eq!(s,s2);
    let s3 = OrderedSetBinTree::from(TEST_STR.chars().collect::<Vec<_>>());
    assert_eq!(s3.to_tree_string(),str1);
    assert_eq!(s3.len(), 14);
    assert_eq!(s3,s2);
    let s4 = OrderedSetBinTree::from(TEST_STR.chars().collect::<BinTree<_>>());
    assert_eq!(s4.to_string(),"[' ', '!', ',', 'H', 'J', 'a', 'e', 'i', 'l', 'm', 'n', 'o', 's', 'y']");
    assert_eq!(s4.len(), 14);
    assert_eq!(s3,s4);
    TEST_STR.chars().for_each(|c| { s.remove(&c); } );
    assert_eq!(s.to_tree_string(),"()");
    assert_eq!(s.len(), 0);
    s.extend(TEST_STR.chars());
    assert_eq!(s.to_tree_string(),str1);
    assert_eq!(s.len(), 14);
    let str2 = s.iter().collect::<String>();
    assert_eq!(str2," !,HJaeilmnosy");
    assert_eq!(s.len(),14);
}
