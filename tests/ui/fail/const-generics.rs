use std::marker::PhantomData;
use variant_partial_eq::VariantPartialEq;

#[derive(VariantPartialEq)]
struct MyStruct<const N: u8> {
    ghost: PhantomData<N>
}

fn main() {}