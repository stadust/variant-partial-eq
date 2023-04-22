use variant_partial_eq::VariantPartialEq;

#[derive(Debug, VariantPartialEq)]
struct MyStruct<'a, 'a_> {
    my_str: &'a_ &'a str
}

fn main() {}