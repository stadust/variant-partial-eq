use std::marker::PhantomData;
use variant_partial_eq::VariantPartialEq;

#[derive(VariantPartialEq)]
struct MyStruct<'a, 'b: 'a> {
    ghost: PhantomData<(&'a str, &'b str)>
}

fn main() {}