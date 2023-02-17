use md_match::derive::MdMatch;
use md_match::{md_match, MdMatch};

#[derive(MdMatch)]
enum A {
    A1(String),
}

#[derive(MdMatch)]
enum B {
    B1(String),
}

#[test]
fn it_works() {
    let (mut a, mut b) = (A::A1(String::from("hello")), B::B1(String::from("world")));

    let (va_ref, vb_ref) = md_match!(&a, &b => |x, y| (x, y));
    assert_eq!(va_ref, "hello");
    assert_eq!(vb_ref, "world");

    md_match!(&mut a, &mut b => |x, y| {
        *x = String::from("world");
        *y = String::from("hello");
    });

    let (va, vb) = md_match!(a, b => |x, y| (x, y));
    assert_eq!(va, "world");
    assert_eq!(vb, "hello");
}
