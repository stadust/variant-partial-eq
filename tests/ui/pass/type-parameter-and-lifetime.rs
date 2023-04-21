use variant_partial_eq::VariantPartialEq;

#[derive(VariantPartialEq)]
struct MyStruct<'a, T> {
    t: &'a T
}

fn main() {}