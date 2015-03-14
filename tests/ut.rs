extern crate tst;

#[cfg(test)]
use tst::tst::*;
#[test]
fn create_root() {
    let m = TST::<i32>::new();
    assert_eq!(0, m.len());
}

#[test]
fn insert() {
    let mut m = TST::<i32>::new();

    assert_eq!(None, m.insert("abc", 13));
    assert_eq!(1, m.len());
}

#[test]
fn insert_2times_without_replace() {
    let mut m = TST::new();
    m.insert("abc", 37);
    assert_eq!(Some(37), m.insert("abc", 666));
    assert_eq!(Some(&666), m.get("abc"));
}

#[test]
fn get() {
    let mut m = TST::new();

    m.insert("abc", 13);
    assert_eq!(Some(&13), m.get("abc"));
}

#[test]
fn get_none() {
    let mut m = TST::new();

    m.insert("abc", 13);
    assert_eq!(None, m.get("abcd"));
    assert_eq!(None, m.get(""));
}

#[test]
fn insert_few() {
    let mut m = TST::new();

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
    let mut m = TST::<i32>::new();

    m.insert("abcde", 13);
    m.insert("abcde", 1);
    assert_eq!(1, m.len());

    assert_eq!(Some(&1), m.get("abcde"));
}

#[test]
fn contains() {
    let mut m = TST::<i32>::new();

    m.insert("xxxe", 13);
    assert!(!m.contains_key("abcde"));
    assert!(!m.contains_key("xxx"));
    assert!(m.contains_key("xxxe"));
}

#[test]
fn is_empty() {
    let mut m = TST::<u32>::new();

    assert_eq!(0, m.len());
    assert!(m.is_empty());

    m.insert("qwer", 10000);
    assert!(!m.is_empty());

    m.clear();
    assert!(m.is_empty());
}

#[test]
fn clear() {
    let mut m = TST::new();
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
    let mut m = TST::<u32>::new();
    assert_eq!(None, m.remove("xxx"));
    assert_eq!(None, m.remove(""));
}

#[test]
fn remove() {
    let mut m = TST::new();
    m.insert("abc", 1);

    assert_eq!(None, m.remove(""));
    assert_eq!(None, m.remove("a"));
    assert_eq!(None, m.remove("ab"));

    assert_eq!(Some(1), m.remove("abc"));

    assert_eq!(None, m.remove("abc"));
}

#[test]
fn longest_prefix_empty() {
    let mut m = TST::new();
    m.insert("abc", 1);

    assert_eq!("", m.longest_prefix("qwer"));
    assert_eq!("", m.longest_prefix(""));
}

#[test]
fn longest_prefix() {
    let mut m = TST::new();
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
    let mut m = TST::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);


    assert_eq!(2, m["abc"]);
    assert_eq!(1, m["abd"]);
    assert_eq!(4, m["abdd"]);
}

#[test]
#[should_fail]
fn access_by_wrong_index() {
    let mut m = TST::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);

    assert_eq!(3, m["a"]);
}

#[test]
fn format_empty() {
    let m = TST::<u64>::new();

    assert_eq!("{}", format!("{:?}", m));
}

#[test]
fn format() {
    let mut m = TST::<i64>::new();

    m.insert("abc", 2);
    m.insert("abd", 1);
    m.insert("abdd", 4);
    m.insert("abcdefghjkik", -169874);

    let m_str = format!("{:?}", m);
    assert_eq!(
        "{\"abc\": 2,\"abcdefghjkik\": -169874,\"abd\": 1,\"abdd\": 4,}", 
        m_str
    );
}

#[test]
fn iterator() {
    let mut m = TST::new();

    m.insert("b", 2);
    m.insert("a", 1);
    m.insert("c", 4);
    m.insert("aa", 13);

    let mut m_str = String::new();
    for x in m.iter() {
        m_str.push_str(format!("{:?}", x).as_slice());
        //println!();
    }
    assert_eq!("(\"a\", 1)(\"aa\", 13)(\"b\", 2)(\"c\", 4)", m_str);
}

/*
#[test]
fn keys() {
    let mut m = TST::<u32>::new();
    m.insert("abc", &1);
    m.insert("bcd", &2);
    m.insert("c", &3);
    m.insert("abcd", &1);

    for key in m.keys() {
        println!("{}", key);
    }
}*/

