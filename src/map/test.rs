use crate::{BinTreeMap};

#[test]
fn test_map() {
    let mut map = BinTreeMap::new();

    map.insert(67, String::from("first value for 67"));
    map.insert(31, String::from("value for 31"));
    map.insert(67, String::from("second value for 67"));

    assert_eq!(map.iter().collect::<Vec<_>>(),
        [(&31, &String::from("value for 31")), (&67, &String::from("second value for 67"))]);

    for (k,v) in &mut map {
        if *k < 50 {
            *v = format!("{} is less than 50",v);
        }
    }

    assert_eq!(map.to_string(),"[(31, \"value for 31 is less than 50\"), (67, \"second value for 67\")]");
    assert_eq!(*map.get(&31).unwrap(),"value for 31 is less than 50");
    assert_eq!(map.get_key_value(&67).unwrap(),(&67,&String::from("second value for 67")));
    assert_eq!(map.contains_key(&67),true);
    assert_eq!(map.contains_key(&167),false);

    assert_eq!(map.to_tree_string(),
        "((\
            BinTreeMapEntry { key: 31, value: \"value for 31 is less than 50\" }) <= \
            BinTreeMapEntry { key: 67, value: \"second value for 67\" }\
        )");
    assert_eq!(map.remove(&31).unwrap(),"value for 31 is less than 50");
    assert_eq!(map.to_string(),"[(67, \"second value for 67\")]");

    assert_eq!(map.into_iter().collect::<Vec<_>>(),vec![(67, "second value for 67".to_owned())]);
}

#[test]
fn check_custom_kv_type() {
    #[derive(PartialEq, PartialOrd, Debug, Default)]
    struct KeyType(i32);

    #[derive(Debug, PartialEq)]
    struct ValueType(i64);

    let mut t : BinTreeMap<KeyType, ValueType> = BinTreeMap::new();

    assert_eq!(t.len(),0);
    t.insert(KeyType(-20), ValueType(782));
    assert_eq!(t.len(),1);
    t.insert(KeyType(3330), ValueType(-1782));
    assert_eq!(t.len(),2);
    t.insert(KeyType(33), ValueType(-14));
    assert_eq!(t.len(),3);
    t.insert(KeyType(33), ValueType(14)); // overwrite entry for key 33
    assert_eq!(t.len(),3);
    t.insert(KeyType(110), ValueType(-1));
    assert_eq!(t.len(),4);
    t.insert(KeyType(-40), ValueType(234));
    assert_eq!(t.len(),5);
    t.insert(KeyType(12), ValueType(82));
    assert_eq!(t.len(),6);
    t.insert(KeyType(130), ValueType(-2));
    assert_eq!(t.len(),7);
    t.insert(KeyType(-876), ValueType(-182));
    assert_eq!(t.len(),8);

    assert_eq!(t.to_tree_string(),"\
                    ((((BinTreeMapEntry { key: KeyType(-876), value: ValueType(-182) }) \
                <= BinTreeMapEntry { key: KeyType(-40), value: ValueType(234) }) \
            <= BinTreeMapEntry { key: KeyType(-20), value: ValueType(782) } => \
                (BinTreeMapEntry { key: KeyType(12), value: ValueType(82) })) \
        <= BinTreeMapEntry { key: KeyType(33), value: ValueType(14) } => \
                ((BinTreeMapEntry { key: KeyType(110), value: ValueType(-1) }) \
            <= BinTreeMapEntry { key: KeyType(130), value: ValueType(-2) } => \
                (BinTreeMapEntry { key: KeyType(3330), value: ValueType(-1782) })))\
    ");

    assert_eq!(t.remove(&KeyType(12)).unwrap(),ValueType(82));
    assert_eq!(t.len(),7);

    assert_eq!(t.to_tree_string(),"\
                    (((BinTreeMapEntry { key: KeyType(-876), value: ValueType(-182) }) \
                <= BinTreeMapEntry { key: KeyType(-40), value: ValueType(234) } => \
                    (BinTreeMapEntry { key: KeyType(-20), value: ValueType(782) })) \
            <= BinTreeMapEntry { key: KeyType(33), value: ValueType(14) } => \
                    ((BinTreeMapEntry { key: KeyType(110), value: ValueType(-1) }) \
                <= BinTreeMapEntry { key: KeyType(130), value: ValueType(-2) } => \
                    (BinTreeMapEntry { key: KeyType(3330), value: ValueType(-1782) })))");

    assert_eq!(format!("{:?}",t.keys().collect::<Vec<_>>()),
        "[KeyType(-876), KeyType(-40), KeyType(-20), KeyType(33), KeyType(110), KeyType(130), KeyType(3330)]");
    assert_eq!(format!("{:?}",t.values().collect::<Vec<_>>()),
        "[ValueType(-182), ValueType(234), ValueType(782), ValueType(14), ValueType(-1), ValueType(-2), ValueType(-1782)]");

    *t.get_mut(&KeyType(110)).unwrap() = ValueType(-110);
    assert_eq!(t.len(),7);
    assert_eq!(t.into_iter().collect::<Vec<_>>(),vec![
        (KeyType(-876), ValueType(-182)), 
        (KeyType(-40), ValueType(234)), 
        (KeyType(-20), ValueType(782)), 
        (KeyType(33), ValueType(14)), 
        (KeyType(110), ValueType(-110)), 
        (KeyType(130), ValueType(-2)), 
        (KeyType(3330), ValueType(-1782))]);
}
