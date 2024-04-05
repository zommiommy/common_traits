use common_traits::*;

#[test]
fn test_invariants() {
    invariant!(true);
    invariant!(true, "this was true! {}", 10);
    invariant!(1 == 1);
    invariant!(1 == 1, "this was true! {}", 10);
    invariant_eq!(1, 1);
    invariant_eq!(1, 1, "one is equal to one {}", 10);
    invariant_ne!(1, 2);
    invariant_ne!(1, 2, "one is not equal to two {}", 10);
}

#[test]
#[should_panic]
fn test_invariant_expr() {
    invariant!(1 == 0, "this was false! {}", 10);
}

#[test]
#[should_panic]
fn test_invariant() {
    invariant!(false, "this was false! {}", 10);
}

#[test]
#[should_panic]
fn test_invariant_eq() {
    invariant_eq!(1, 2, "one is not equal to two {}", 10);
}

#[test]
#[should_panic]
fn test_invariant_ne() {
    invariant_ne!(1, 1, "one is not equal to one {}", 10);
}
