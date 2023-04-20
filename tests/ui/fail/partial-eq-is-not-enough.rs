trait ThunkProcessor {
    type Output<'a>;
}

#[derive(Debug)]
enum Thunk<'a, P: ThunkProcessor> {
    Unprocessed(&'a str),
    Processed(P::Output<'a>)
}

impl<'a, 'b, P: ThunkProcessor> PartialEq<Thunk<'b, P>> for Thunk<'a, P>
    where
        P::Output<'a>: PartialEq<P::Output<'b>>
{
    fn eq(&self, other: &Thunk<'b, P>) -> bool {
        match (self, other) {
            (Thunk::Processed(o1), Thunk::Processed(o2)) => o1 == o2,
            (Thunk::Unprocessed(s1), Thunk::Unprocessed(s2)) => s1 == s2,
            _ => false
        }
    }
}

#[derive(Debug)]
struct Processor;

impl ThunkProcessor for Processor {
    type Output<'a> = &'a str;
}

#[derive(Debug, PartialEq)]
struct ThunkHolder<'a> {
    concrete_thunk: Thunk<'a, Processor>,
}

fn main() {
    let s = "Hello World!".to_string();

    let h1 = ThunkHolder::<'static> {
        concrete_thunk: Thunk::Unprocessed("Hello World!")
    };
    let h2 = ThunkHolder {
        concrete_thunk: Thunk::Unprocessed(s.as_ref())
    };

    assert_eq!(h1, h2)
}