use variant_partial_eq::VariantPartialEq;

#[derive(Debug, VariantPartialEq)]
struct MyStruct<'a> {
    #[variant_compare = "nonsense"]
    #[variant_compare = "nonsense2"]
    my_str: &'a str
}

fn main() {
    let s = "Hello World!".to_owned();

    assert_eq!(MyStruct {my_str: &s}, MyStruct {my_str: "Hello World!"});
}