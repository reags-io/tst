//! Ternary search trie (TST) container.

/// - Create a `TSTMap` containing a given list of elements:
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate tst;
/// # fn main() {
/// let m = tstmap!{
///     "b" => 2, "a" => -1, "c" => 3,
/// };
///
/// assert_eq!(3, m.len());
/// assert_eq!(m["a"], -1);
/// assert_eq!(m["b"], 2);
/// assert_eq!(m["c"], 3);
/// # }
/// ```
#[macro_export]
macro_rules! tstmap {
    () => {{
        $crate::TSTMap::new()
    }};
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (tstmap!($($key => $value),+));
    ($( $key: expr => $val: expr ),*) => {{
        let mut m = $crate::TSTMap::new();
        $(
            m.insert($key, $val);
        )*
        m
    }};
}

extern crate core;

pub mod map;
/// TST container map and set implementation.
pub mod node;
pub mod traverse;

pub use map::TSTMap;
