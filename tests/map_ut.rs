#[macro_use]
extern crate tst;

use self::tst::map::Entry::*;
use self::tst::TSTMap;
use std::iter::FromIterator;

fn prepare_data() -> TSTMap<i32> {
    tstmap! {
        "BY" => 1,
        "BYGONE" => 3,
        "BYE" => 2,
        "BYLAW" => 4,
        "BYLINE" => 5,
        "BYPASS" => 6,
        "BYPATH" => 7,
        "BYPRODUCT" => 8,
        "BYROAD" => 9,
        "BYSTANDER" => 10,
        "BYTE" => 11,
        "BYWAY" => 12,
        "BYWORD" => 13,
    }
}

#[test]
fn create_root() {
    let m = TSTMap::<i32>::new();
    assert_eq!(0, m.len());
}

#[test]
fn map_clone() {
    let orig = tstmap! {
        "first" => 1,
        "second" => 2,
        "firstthird" => 3,
        "firstsecond" => 12,
    };

    let cpy = orig.clone();
    assert_eq!(orig, cpy);
}

#[test]
fn map_clone_strings_as_value() {
    let orig = tstmap! {
        "first" => "1",
        "second" => "2",
        "firstthird" => "3",
        "firstsecond" => "12",
    };

    let cpy = orig.clone();
    assert_eq!(orig, cpy);
}

#[test]
fn insert() {
    let mut m = TSTMap::<i32>::new();

    assert_eq!(None, m.insert("abc", 13));
    assert_eq!(1, m.len());
}

#[test]
fn insert_string_as_value() {
    let mut m = TSTMap::<String>::new();

    assert_eq!(None, m.insert("abc", "xxxx".to_string()));
    assert_eq!("xxxx", m["abc"]);
    assert_eq!(1, m.len());
}

#[test]
fn insert_vec_as_value() {
    let mut m = TSTMap::<Vec<String>>::new();

    assert_eq!(
        None,
        m.insert("abc", vec!["xxxx".to_owned(), "1234STRING".to_owned()])
    );
    assert_eq!(vec!["xxxx".to_owned(), "1234STRING".to_owned()], m["abc"]);
    assert_eq!(1, m.len());
}

#[test]
fn insert_2times_without_replace() {
    let mut m = TSTMap::new();
    m.insert("abc", 37);
    assert_eq!(Some(37), m.insert("abc", 666));
    assert_eq!(Some(&666), m.get("abc"));
}

#[test]
fn get() {
    let mut m = TSTMap::new();

    m.insert("abc", 13);
    assert_eq!(Some(&13), m.get("abc"));
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_none() {
    let mut m = TSTMap::new();

    m.insert("abc", 13);
    assert_eq!(None, m.get("abcd"));
    assert_eq!(None, m.get(""));
}

#[test]
fn get_mut() {
    let mut m = TSTMap::new();
    m.insert("abc", 1);
    match m.get_mut("abc") {
        Some(x) => *x = 13,
        None => panic!(),
    }
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_mut_none_empty_map() {
    let mut m = TSTMap::<u32>::new();
    assert_eq!(None, m.get_mut("x"));
}

#[test]
fn get_mut_none() {
    let mut m = TSTMap::new();
    m.insert("abc", 1);

    assert_eq!(None, m.get_mut("abx"));
}

#[test]
fn entry_occupied() {
    let mut m = TSTMap::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(mut entry) => {
            assert_eq!(&14, entry.get());
            assert_eq!(14, entry.insert(100));
        }
    }
    assert_eq!(Some(&100), m.get("abcdf"));
}

#[test]
fn entry_occupied_remove() {
    let mut m = TSTMap::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(entry) => {
            assert_eq!(14, entry.remove());
        }
    }
    assert_eq!(None, m.get("abcdf"));
    assert_eq!(1, m.len());
}

#[test]
fn entry_occupied_update() {
    let mut m = TSTMap::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    match m.entry("abcdf") {
        Vacant(_) => unreachable!(),
        Occupied(mut entry) => {
            {
                let v = entry.get_mut();
                assert_eq!(14, *v);
                *v += 100;
            }
            {
                let v = entry.get_mut();
                assert_eq!(114, *v);
                *v += 100;
            }
        }
    }
    assert_eq!(Some(&214), m.get("abcdf"));
    assert_eq!(2, m.len());
}

#[test]
fn entry_vacant() {
    let mut m = TSTMap::new();

    match m.entry("abcdg") {
        Vacant(entry) => {
            assert_eq!(100, *entry.insert(100));
        }
        Occupied(_) => unreachable!(),
    }
    assert_eq!(Some(&100), m.get("abcdg"));
    assert_eq!(1, m.len());
}

#[test]
fn insert_few() {
    let mut m = TSTMap::new();

    m.insert("abcde", 13);
    m.insert("abcdf", 14);
    m.insert("abcdg", 15);
    assert_eq!(3, m.len());

    assert_eq!(Some(&13), m.get("abcde"));
    assert_eq!(Some(&14), m.get("abcdf"));
    assert_eq!(Some(&15), m.get("abcdg"));
    assert_eq!(None, m.get("abcdh"));
}

#[test]
fn replace() {
    let mut m = TSTMap::<i32>::new();

    m.insert("abcde", 13);
    m.insert("abcde", 1);
    assert_eq!(1, m.len());

    assert_eq!(Some(&1), m.get("abcde"));
}

#[test]
fn contains() {
    let mut m = TSTMap::<i32>::new();

    m.insert("xxxe", 13);
    assert!(!m.contains_key("abcde"));
    assert!(!m.contains_key("xxx"));
    assert!(m.contains_key("xxxe"));
}

#[test]
fn is_empty() {
    let mut m = TSTMap::<u32>::new();

    assert_eq!(0, m.len());
    assert!(m.is_empty());

    m.insert("qwer", 10_000);
    assert!(!m.is_empty());

    m.clear();
    assert!(m.is_empty());
}

#[test]
fn clear() {
    let mut m = TSTMap::new();
    m.clear();
    assert_eq!(None, m.insert("abc", 11));
    assert_eq!(None, m.insert("abcd", -3));
    assert_eq!(None, m.insert("a", 2));
    m.clear();
    assert_eq!(None, m.insert("abc", 11));
    assert_eq!(None, m.insert("abcd", -3));
    assert_eq!(None, m.insert("a", 2));
}

#[test]
fn remove_from_empty() {
    let mut m = TSTMap::<u32>::new();

    assert_eq!(None, m.remove("xxx"));
    assert_eq!(None, m.remove(""));
    assert_eq!(0, m.len());
}

#[test]
fn remove_non_existing() {
    let mut m = tstmap!["abc" => 1];

    assert_eq!(None, m.remove(""));
    assert_eq!(None, m.remove("a"));
    assert_eq!(None, m.remove("ab"));
}

#[test]
fn remove() {
    let mut m = tstmap!["abc" => 1];

    assert_eq!(Some(1), m.remove("abc"));
    assert_eq!(None, m.get("abc"));
    assert_eq!(None, m.remove("abc"));
    assert_eq!(None, m.get("abc"));
    assert_eq!(0, m.len());
}

#[test]
fn remove_rich() {
    let mut m = prepare_data();

    assert_eq!(Some(1), m.remove("BY"));
    assert_eq!(Some(12), m.remove("BYWAY"));
    assert_eq!(Some(10), m.remove("BYSTANDER"));
    assert_eq!(Some(8), m.remove("BYPRODUCT"));
    assert_eq!(Some(2), m.remove("BYE"));
    assert_eq!(8, m.len());
}

#[test]
fn remove_only_tail() {
    let mut m = prepare_data();

    assert_eq!(Some(8), m.remove("BYPRODUCT"));

    assert_eq!(1, m["BY"]);
    assert_eq!(2, m["BYE"]);
    assert_eq!(12, m.len());
}

#[test]
fn longest_prefix_empty() {
    let mut m = TSTMap::new();
    m.insert("abc", 1);

    assert_eq!("", m.longest_prefix("qwer"));
    assert_eq!("", m.longest_prefix(""));
}

#[test]
fn longest_prefix() {
    let mut m = TSTMap::new();
    m.insert("abc", 1);
    m.insert("abcd", 1);
    m.insert("abce", 1);
    m.insert("abca", 1);
    m.insert("zxd", 1);
    m.insert("add", 1);
    m.insert("abcdef", 1);

    assert_eq!("abcd", m.longest_prefix("abcde"));
    assert_eq!("abcdef", m.longest_prefix("abcdef"));
}

#[test]
fn access_by_index() {
    let mut m = TSTMap::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);

    assert_eq!(2, m["abc"]);
    assert_eq!(1, m["abd"]);
    assert_eq!(4, m["abdd"]);
}

#[test]
fn access_by_index_mut() {
    let mut m = TSTMap::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);
    {
        let v = &mut m["abc"];
        *v += 1;
    }

    assert_eq!(3, m["abc"]);
    assert_eq!(1, m["abd"]);
    assert_eq!(4, m["abdd"]);
}

#[test]
#[should_panic]
fn access_by_wrong_index() {
    let mut m = TSTMap::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);

    assert_eq!(3, m["a"]);
}

#[test]
fn format_empty() {
    let m = TSTMap::<u64>::new();

    assert_eq!("{}", format!("{:?}", m));
}

#[test]
fn format() {
    let mut m = TSTMap::<i64>::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);
    m.insert("abcdefghjkik", -169_874);

    let m_str = format!("{:?}", m);
    assert_eq!(
        "{\"abc\": 2, \"abcdefghjkik\": -169874, \"abd\": 1, \"abdd\": 4}",
        m_str
    );
}

#[test]
fn iterator() {
    let mut m = TSTMap::new();

    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 4);
    m.insert("aa", 13);

    let mut m_str = String::new();
    for x in m.iter() {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"a\", 1)(\"aa\", 13)(\"b\", 2)(\"c\", 4)", m_str);
}

#[test]
fn iterator_mut() {
    let mut m = TSTMap::new();

    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 4);
    m.insert("aa", 13);

    for (_, v) in m.iter_mut() {
        *v *= 3;
    }
    assert_eq!(Some(&6), m.get("b"));
    assert_eq!(Some(&3), m.get("a"));
    assert_eq!(Some(&12), m.get("c"));
    assert_eq!(Some(&39), m.get("aa"));
}

#[test]
fn into_iter() {
    let m = tstmap! {
        "b" => 2,
        "a" => 1,
        "c" => 4,
        "aa" => 13,
    };
    let vec = m.into_iter().collect::<Vec<(String, i32)>>();
    let orig = vec![
        ("a".to_string(), 1),
        ("aa".to_string(), 13),
        ("b".to_string(), 2),
        ("c".to_string(), 4),
    ];
    assert_eq!(orig, vec);
}

#[test]
fn from_iterator_empty() {
    let vec = vec![];
    let m = TSTMap::<i64>::from_iter(vec);

    assert_eq!(true, m.is_empty());
}

#[test]
fn from_iterator() {
    let vec = vec![("b", 2), ("a", 1), ("c", 4), ("a", 100), ("aa", 13)];

    let m = TSTMap::from_iter(vec);
    let orig = tstmap! {
        "b" => 2,
        "c" => 4,
        "a" => 100,
        "aa" => 13,
    };
    assert_eq!(orig, m);
}

#[test]
fn extend() {
    let mut m = tstmap! {
        "a" => 13,
    };
    let vec = vec![("b", 2), ("a", 1), ("c", 4), ("a", 100), ("aa", 13)];
    m.extend(vec);
    let orig = tstmap! {
        "a" => 100,
        "b" => 2,
        "c" => 4,
        "aa" => 13,
    };
    assert_eq!(orig, m);
}

#[test]
fn prefix_iterator_empty() {
    let mut m = TSTMap::new();

    m.insert("bbc", 2);
    m.insert("abc", 1);
    m.insert("dbc", 4);

    let mut m_str = String::new();
    for x in m.prefix_iter("abd") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("", m_str);
}

#[test]
fn prefix_iterator() {
    let mut m = TSTMap::new();

    m.insert("first", 1);
    m.insert("second", 2);
    m.insert("firstthird", 3);
    m.insert("firstsecond", 12);

    let mut m_str = String::new();

    for x in m.prefix_iter("fir") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!(
        "(\"first\", 1)(\"firstsecond\", 12)(\"firstthird\", 3)",
        m_str
    );
}

#[test]
fn prefix_iterator_only_one() {
    let m = prepare_data();
    let mut m_str = String::new();

    for x in m.prefix_iter("BYE") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"BYE\", 2)", m_str);
}

#[test]
fn prefix_iterator_mut() {
    let mut m = TSTMap::new();
    m.insert("first", 1);
    m.insert("second", 2);
    m.insert("firstthird", 3);
    m.insert("firstsecond", 12);

    for x in m.prefix_iter_mut("fir") {
        *x.1 -= 13;
    }
    assert_eq!(Some(&-12), m.get("first"));
    assert_eq!(Some(&2), m.get("second"));
    assert_eq!(Some(&-10), m.get("firstthird"));
    assert_eq!(Some(&-1), m.get("firstsecond"));
}

#[test]
fn prefix_iterator_mut_empty() {
    let orig = tstmap! {
        "first" => 1,
        "second" => 2,
        "firstthird" => 3,
        "firstsecond" => 12,
    };

    let mut m = orig.clone();
    for x in m.prefix_iter_mut("third!") {
        *x.1 -= 13;
    }
    assert_eq!(orig, m);
}

#[test]
fn keys_iterator() {
    let mut m = TSTMap::new();
    m.insert("abc", 1);
    m.insert("bcd", 2);
    m.insert("c", 3);
    m.insert("abcd", 1);

    let mut m_str = String::new();

    for k in m.keys() {
        m_str.push_str(&format!("{:?}", k));
    }
    assert_eq!("\"abc\"\"abcd\"\"bcd\"\"c\"", m_str);
}

#[test]
fn values_iterator() {
    let m = tstmap! {
        "abc" => 1,
        "bcd" => 2,
        "c" => 3,
        "abcd" => 13,
        "xxx" => 130,
    };

    let mut m_str = String::new();

    for v in m.values() {
        m_str.push_str(&format!("{:?} ", v));
    }
    assert_eq!("1 13 2 3 130 ", m_str);
}

#[test]
fn wildcard_iter_simple() {
    let m = tstmap! {
        "x" => 1,
        "y" => 2,
    };

    let mut m_str = String::new();

    for x in m.wildcard_iter(".") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"x\", 1)(\"y\", 2)", m_str);
}

#[test]
fn wildcard_iter() {
    let m = prepare_data();

    let mut m_str = String::new();
    for x in m.wildcard_iter("BYPA..") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"BYPASS\", 6)(\"BYPATH\", 7)", m_str);
}

#[test]
fn wildcard_iter_dot_at_begin() {
    let m = tstmap! {
        "bac" => 1,
        "aac" => 2,
        "cac" => 3,
    };

    let mut m_str = String::new();

    for x in m.wildcard_iter(".ac") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"aac\", 2)(\"bac\", 1)(\"cac\", 3)", m_str);
}

#[test]
fn wildcard_iter_dot_at_end() {
    let m = tstmap! {
        "bac" => 1,
        "aac" => 2,
        "bax" => 3,
    };

    let mut m_str = String::new();

    for x in m.wildcard_iter("ba.") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("(\"bac\", 1)(\"bax\", 3)", m_str);
}

#[test]
fn wildcard_iter_empty() {
    let m = tstmap! {
        "BY" => 1,
        "BYE" => 2,
    };

    let mut m_str = String::new();

    for x in m.wildcard_iter("BY..") {
        m_str.push_str(&format!("{:?}", x));
    }
    assert_eq!("", m_str);
}

#[test]
fn wildcard_iter_mut() {
    let mut m = prepare_data();

    for (_, v) in m.wildcard_iter_mut("BYPA..") {
        *v = -13;
    }
    assert_eq!(-13, m["BYPASS"]);
    assert_eq!(-13, m["BYPATH"]);
    assert_eq!(8, m["BYPRODUCT"]);
    assert_eq!(5, m["BYLINE"]);
}

#[test]
fn wildcard_iter_unicode() {
    let mut m = TSTMap::new();
    m.insert("ухонос", 100);
    m.insert("сухонос", 1000);
    m.insert("хонос", 10000);
    m.insert("онос", -100);

    let mut m_str = String::new();
    for x in m.wildcard_iter("..х.но.") {
        m_str.push_str(&format!("{:?}", x));
    }

    assert_eq!("(\"сухонос\", 1000)", m_str);
}

#[test]
fn eq_empty() {
    let m1 = TSTMap::<i32>::new();
    let m2 = TSTMap::<i32>::new();
    assert_eq!(m1, m2);
}

#[test]
fn eq_non_empty() {
    let mut m1 = TSTMap::<i32>::new();
    let mut m2 = TSTMap::<i32>::new();

    m2.insert("abcdef", 100);
    m2.insert("xxx", 2);
    m1.insert("abcdef", 100);
    m1.insert("xxx", 2);

    assert_eq!(m1, m2);
}

#[test]
fn not_eq() {
    let m1 = TSTMap::<i32>::new();
    let mut m2 = TSTMap::<i32>::new();

    m2.insert("xxx", 2);

    assert!(m1 != m2);
}

#[test]
fn not_eq_only_value() {
    let mut m1 = TSTMap::<i32>::new();
    let mut m2 = TSTMap::<i32>::new();

    m2.insert("abcdef", 100);
    m2.insert("xxx", 2);
    m1.insert("abcdef", -100);
    m1.insert("xxx", 2);

    assert!(m1 != m2);
}

#[test]
fn macros_ctor_empty() {
    let m: TSTMap<u64> = tstmap![];

    assert_eq!(0, m.len());
    assert_eq!(None, m.get("abc"));
}

#[test]
fn macros_ctor() {
    let m = tstmap!["x" => -100, "a" => 13, "abc" => 0, "z" => 2, "abcd" => 666];

    assert_eq!(5, m.len());
    assert_eq!(Some(&13), m.get("a"));
    assert_eq!(None, m.get("ab"));
    assert_eq!(Some(&0), m.get("abc"));
    assert_eq!(Some(&-100), m.get("x"));
    assert_eq!(Some(&2), m.get("z"));
    assert_eq!(Some(&666), m.get("abcd"));
}

#[test]
fn insert_remove_get_big_key_not_overflow_stack() {
    let mut m = TSTMap::new();
    let mut key = String::new();

    while key.len() < 1_000_000 {
        key.push_str("qwertyuiopasdfghjkl;");
    }
    m.insert(&key, 666);

    assert_eq!(1, m.len());
    assert_eq!(Some(&666), m.get(&key));
    assert_eq!(Some(666), m.remove(&key));
    assert_eq!(None, m.get(&key));
}

#[test]
fn drop_stack_overflow() {
    let mut m = TSTMap::new();
    let mut key = String::new();

    while key.len() < 1_000_000 {
        key.push_str("qwertyuiopasdfghjkl;");
    }
    m.insert(&key, 666);

    assert_eq!(1, m.len());
}

#[test]
fn unicode() {
    let mut m = TSTMap::new();
    m.insert("::ХУЙ", 12);

    assert_eq!(1, m.len());
    assert_eq!(None, m.get("::ХУЙЯ"));
    assert_eq!(Some(&12), m.get("::ХУЙ"));
    assert_eq!(Some(12), m.remove("::ХУЙ"));
    assert_eq!(None, m.get("::ХУЙ"));
}
