//! # md-match
//!
//! A macro to support md-match.
//!
//! ```rust
//! use md_match::derive::MdMatch;
//! use md_match::{md_match, MdMatch};
//!
//! #[derive(MdMatch)]
//! enum A {
//!     A1(String),
//! }
//!
//! #[derive(MdMatch)]
//! enum B {
//!     B1(String),
//! }
//!
//! let (mut a, mut b) = (A::A1(String::from("hello")), B::B1(String::from("world")));
//!
//! let (va_ref, vb_ref) = md_match!(&a, &b => |x, y| (x, y));
//! assert_eq!(va_ref, "hello");
//! assert_eq!(vb_ref, "world");
//!
//! md_match!(&mut a, &mut b => |x, y| {
//!     *x = String::from("world");
//!     *y = String::from("hello");
//! });
//!
//! let (va, vb) = md_match!(a, b => |x, y| (x, y));
//! assert_eq!(va, "world");
//! assert_eq!(vb, "hello");
//! ```

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
