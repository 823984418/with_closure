macro_rules! with_define {
    ($name:ident $(,$arg:ident)*) => {
        #[doc(hidden)]
        #[inline(always)]
        #[allow(non_snake_case)]
        pub fn $name<$($arg,)* F: FnOnce($($arg,)*) -> R, R>($($arg: $arg,)* f: F) -> R {
            f($($arg,)*)
        }
    };
}

with_define!(with_0);
with_define!(with_1, A0);
with_define!(with_2, A0, A1);
with_define!(with_3, A0, A1, A2);
with_define!(with_4, A0, A1, A2, A3);
with_define!(with_5, A0, A1, A2, A3, A4);
with_define!(with_6, A0, A1, A2, A3, A4, A5);
with_define!(with_7, A0, A1, A2, A3, A4, A5, A6);
with_define!(with_8, A0, A1, A2, A3, A4, A5, A6, A7);
with_define!(with_9, A0, A1, A2, A3, A4, A5, A6, A7, A8);
with_define!(with_10, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
with_define!(with_11, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
with_define!(with_12, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);

#[doc(hidden)]
#[inline(always)]
pub fn with_more<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A, F: FnOnce(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A) -> R, R>(a0: A0, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5, a6: A6, a7: A7, a8: A8, a9: A9, a10: A10, a11: A11, a: A, f: F) -> R {
    f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a)
}

#[macro_export]
macro_rules! with {
    ($f:block) => {
        $crate::with_0(|| $f)
    };
    ($a0:pat = $va0:expr, $f:block) => {
        $crate::with_1($va0, |$a0| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $f:block) => {
        $crate::with_2($va0, $va1, |$a0, $a1| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $f:block) => {
        $crate::with_3($va0, $va1, $va2, |$a0, $a1, $a2| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $f:block) => {
        $crate::with_4($va0, $va1, $va2, $va3, |$a0, $a1, $a2, $a3| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $f:block) => {
        $crate::with_5($va0, $va1, $va2, $va3, $va4, |$a0, $a1, $a2, $a3, $a4| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $f:block) => {
        $crate::with_6($va0, $va1, $va2, $va3, $va4, $va5, |$a0, $a1, $a2, $a3, $a4, $a5| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $f:block) => {
        $crate::with_6($va0, $va1, $va2, $va3, $va4, $va5, |$a0, $a1, $a2, $a3, $a4, $a5| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $f:block) => {
        $crate::with_7($va0, $va1, $va2, $va3, $va4, $va5, $va6, |$a0, $a1, $a2, $a3, $a4, $a5, $a6| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $f:block) => {
        $crate::with_8($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $a8:pat = $va8:expr, $f:block) => {
        $crate::with_9($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, $va8, |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $a8:pat = $va8:expr, $a9:pat = $va9:expr, $f:block) => {
        $crate::with_10($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, $va8, $va9, |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $a8:pat = $va8:expr, $a9:pat = $va9:expr, $a10:pat = $va10:expr, $f:block) => {
        $crate::with_11($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, $va8, $va9, $va10, |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $a8:pat = $va8:expr, $a9:pat = $va9:expr, $a10:pat = $va10:expr, $a11:pat = $va11:expr, $f:block) => {
        $crate::with_12($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, $va8, $va9, $va10, $va11, |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10, $a11| $f)
    };
    ($a0:pat = $va0:expr, $a1:pat = $va1:expr, $a2:pat = $va2:expr, $a3:pat = $va3:expr, $a4:pat = $va4:expr, $a5:pat = $va5:expr, $a6:pat = $va6:expr, $a7:pat = $va7:expr, $a8:pat = $va8:expr, $a9:pat = $va9:expr, $a10:pat = $va10:expr, $a11:pat = $va11:expr, $($a:pat = $va:expr,)* $f:block) => {
        $crate::with_more($va0, $va1, $va2, $va3, $va4, $va5, $va6, $va7, $va8, $va9, $va10, $va11, ($($va,)*), |$a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10, $a11, ($($a,)*)| $f)
    };
}
