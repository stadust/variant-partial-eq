use std::marker::PhantomData;
use variant_partial_eq::VariantPartialEq;

#[derive(VariantPartialEq)]
struct MyStruct<T> {
    ghost: PhantomData<T>
}

fn main() {}