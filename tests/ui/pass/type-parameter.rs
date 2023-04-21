use variant_partial_eq::VariantPartialEq;

#[derive(VariantPartialEq)]
struct MyStruct<T> {
    t: T
}

fn main() {}