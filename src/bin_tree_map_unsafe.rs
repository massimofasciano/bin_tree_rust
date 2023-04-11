use crate::BinTreeMap;

impl<Key : PartialOrd, Value> BinTreeMap<Key,Value> {
    /// swap 2 items in the map (lookup by key)
    pub fn swap(&mut self, key1 : &Key, key2 : &Key) -> bool {
        let opt1 = self.get_mut(key1);
        if opt1.is_none() { return false }
        let ptr1 = opt1.unwrap() as * mut Value;
        let opt2 = self.get_mut(key2);
        if opt2.is_none() { return false }
        let ptr2 = opt2.unwrap() as * mut Value;
        if ptr1 != ptr2 {
            unsafe {
                std::ptr::swap(ptr1,ptr2);
            }
            true
        } else {
            false
        }
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTreeMap};

    #[test]
    fn test_map_swap() {
        #[derive(PartialEq, PartialOrd, Debug, Default)]
        struct KeyType(i32);

        #[derive(Debug, Default, PartialEq)]
        struct ValueType(i64);

        let mut t : BinTreeMap<KeyType, ValueType> = BinTreeMap::new();

        t.insert(KeyType(-20), ValueType(782));
        t.insert(KeyType(3330), ValueType(-1782));
        t.insert(KeyType(33), ValueType(14));
        t.insert(KeyType(110), ValueType(-1));
        t.insert(KeyType(-40), ValueType(234));
        t.insert(KeyType(12), ValueType(82));
        t.insert(KeyType(130), ValueType(-2));
        t.insert(KeyType(-876), ValueType(-182));

        assert_eq!(t.to_string(),"[\
            (KeyType(-876), ValueType(-182)), \
            (KeyType(-40), ValueType(234)), \
            (KeyType(-20), ValueType(782)), \
            (KeyType(12), ValueType(82)), \
            (KeyType(33), ValueType(14)), \
            (KeyType(110), ValueType(-1)), \
            (KeyType(130), ValueType(-2)), \
            (KeyType(3330), ValueType(-1782))\
        ]");

        assert_eq!(t.swap(&KeyType(3330), &KeyType(130)),true);
        assert_eq!(t.swap(&KeyType(12), &KeyType(-40)),true);
        assert_eq!(t.swap(&KeyType(120), &KeyType(-40)),false);
        assert_eq!(t.swap(&KeyType(12), &KeyType(-400)),false);
        assert_eq!(t.swap(&KeyType(12), &KeyType(12)),false);

        assert_eq!(t.to_string(),"[\
            (KeyType(-876), ValueType(-182)), \
            (KeyType(-40), ValueType(82)), \
            (KeyType(-20), ValueType(782)), \
            (KeyType(12), ValueType(234)), \
            (KeyType(33), ValueType(14)), \
            (KeyType(110), ValueType(-1)), \
            (KeyType(130), ValueType(-1782)), \
            (KeyType(3330), ValueType(-2))\
        ]");

    }
}