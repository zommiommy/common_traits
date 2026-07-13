use common_traits::*;

#[test]
fn test_invariants() {
    // SAFETY: every condition asserted below is trivially true, so the
    // invariants hold in both debug and release builds.
    unsafe {
        invariant!(true);
        invariant!(true, "this was true! {}", 10);
        invariant!(1 == 1);
        invariant!(1 == 1, "this was true! {}", 10);
        invariant_eq!(1, 1);
        invariant_eq!(1, 1, "one is equal to one {}", 10);
        invariant_ne!(1, 2);
        invariant_ne!(1, 2, "one is not equal to two {}", 10);
    }
}

// The `should_panic` tests below deliberately pass a false condition to check
// that debug builds panic. They rely on `debug_assertions` being enabled (the
// default `cargo test` profile); the `unsafe` blocks are sound only under that
// profile and must not be run in release.

#[test]
#[should_panic]
fn test_invariant_expr() {
    unsafe {
        invariant!(1 == 0, "this was false! {}", 10);
    }
}

#[test]
#[should_panic]
fn test_invariant() {
    unsafe {
        invariant!(false, "this was false! {}", 10);
    }
}

#[test]
#[should_panic]
fn test_invariant_eq() {
    unsafe {
        invariant_eq!(1, 2, "one is not equal to two {}", 10);
    }
}

#[test]
#[should_panic]
fn test_invariant_ne() {
    unsafe {
        invariant_ne!(1, 1, "one is not equal to one {}", 10);
    }
}
