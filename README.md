# A #[derive] macro for PartialEq implementations that ignore lifetime variance

Consider a type such as 

```rust
pub struct StrWrap<'a> {
    wrapped: &'a str
}
```

The standard `#[derive(PartialEq)]` generates an implementation as follows

```rust
impl<'a> PartialEq<StrWrap<'a>> for StrWrap<'a> { ... }
```

This seems to imply that we can only compare `StrWrap`s where the lifetime of the wrapped strings are identical. Nevertheless, code such as

```rust
static STATIC_STR: &'static str = "Hello World!";

let local_str = String::from("Hellow World!");
let wrap1: StrWrap<'static> = StrWrap { wrapped: STATIC_STR };
let wrap2 = StrWrap { wrapped: local_str.as_ref() }; // <-- lets call the anonymous lifetime here '1
assert_eq!(wrap1, wrap2);
```

compiles and runs just fine, even though the wrapped str's clearly have different lifetimes! Why? The lifetime `'a` is *covariant*: If I have some other lifetime `'b: 'a`, then any place that wants a `'a` happily accepts a `'b`. Or, in our specific case, we see that `PartialEq` wants two `StrWrap<'1>`, but since `'static` outlives `'1`, the compiler realises that downgrading the `'static` to a `'1` for the duration of the call to `.eq` is perfectly fine. This is called [*subtyping*](https://doc.rust-lang.org/nomicon/subtyping.html).

However, what if my lifetime is not covariant? This happens for example with lifetimes used in generic associated types (GATs). In this case, the compiler cannot adjust our lifetimes for us, and the above example will indeed fail to compile, as there is no `PartialEq` implementation for `StrWrap<'1> == StrWrap<'static>`, only for `StrWrap<'1> == StrWrap<'1>` and `StrWrap<'static> == StrWrap<'static>`. However, being unable to compare two values because the lifetimes are different is a bit silly, especially since lifetimes are not actually a thing at runtime. 

We can work around this limitation by hand-rolling our own `PartialEq` instead of using the build-in derive:

```rust
impl<'a, 'b> PartialEq<StrWrap<'b>> for StrWrap<'a> {
    fn eq(&self, other: &StrWrap<'b>) -> bool {
        self.wrapped == other.wrapped
    }
}
```

With this, our example works again, as now `StrWrap<'1> == StrWrap<'static>` is possible. Generating these generalized `PartialEq` implementations is exactly the purpose of this crate. 