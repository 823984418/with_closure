#[doc(hidden)]
#[inline(always)]
pub fn with<A, F: FnOnce(A) -> R, R>(a: A, f: F) -> R {
    f(a)
}

#[macro_export]
macro_rules! with {
    ($($a:pat = $va:expr,)* $f:block) => {
        $crate::with(($($va,)*), |($($a,)*)| $f)
    };
}
