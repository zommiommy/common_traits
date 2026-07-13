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

// The `should_panic` tests below deliberately pass a false condition. The
// `invariant*!` macros only `assert!` (and thus panic) while `debug_assertions`
// are enabled; in release builds a false condition reaches
// `core::hint::unreachable_unchecked()`, which is undefined behavior. These
// tests are therefore compiled only in debug builds.

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_invariant_expr() {
    unsafe {
        // SAFETY: this test is `debug_assertions`-only, where the macro's
        // `assert!` panics on the false condition before reaching
        // `unreachable_unchecked`, so the invalid invariant never causes UB.
        invariant!(1 == 0, "this was false! {}", 10);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_invariant() {
    unsafe {
        // SAFETY: debug-only; the macro's `assert!` panics on the false
        // condition before `unreachable_unchecked` is reached.
        invariant!(false, "this was false! {}", 10);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_invariant_eq() {
    unsafe {
        // SAFETY: debug-only; the macro's `assert!` panics on the false
        // condition before `unreachable_unchecked` is reached.
        invariant_eq!(1, 2, "one is not equal to two {}", 10);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_invariant_ne() {
    unsafe {
        // SAFETY: debug-only; the macro's `assert!` panics on the false
        // condition before `unreachable_unchecked` is reached.
        invariant_ne!(1, 1, "one is not equal to one {}", 10);
    }
}
