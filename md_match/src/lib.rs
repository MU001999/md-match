#![doc = include_str!("../../README.md")]

/// derive macro for MdMatch
pub mod derive {
    /// derive macro for MdMatch
    pub use md_match_derive::MdMatch;
}

/// trait for md-match
pub trait MdMatch {
    /// element type
    type Elem;
    /// match the element of MdMatch
    fn md_match<R>(self, f: impl FnOnce(Self::Elem) -> R) -> R;
}

/// match the element of MdMatch
#[macro_export]
macro_rules! md_match {
    ($e: expr => |$var: pat_param| $body: expr) => {
        ($e).md_match(|$var| $body)
    };
    ($e: expr, $($es: expr),+ => |$var: pat_param, $($vars: pat_param),+| $body: expr) => {
        ($e).md_match(|$var| md_match!($($es),* => |$($vars),*| $body))
    };
}
