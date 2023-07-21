# With Closure
Ensure that the `noalias` optimization takes effect by expanding to closure call.

### Implementation
This library only contains one macro definition:
```rust
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
```

### Reason and usage
Due to compiler limitations, some code cannot achieve complete alias optimization.
```rust
pub fn foo(mut x: Vec<i32>) {
    x[0] = 1;
    println!("do something");
    if x[0] != 1 {
        println!("branch");
    }
}
```
The compiler cannot delete the branch.

After passing a function, the compiler learned about this.
```rust
pub fn foo(mut x: Vec<i32>) {
    with!(x = x.as_mut_slice(), {
        x[0] = 1;
        println!("do something");
        if x[0] != 1 {
            println!("branch");
        }
    });
}
```
Branch deleted.

