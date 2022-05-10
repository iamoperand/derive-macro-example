use trait_derive::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct Foo;

#[test]
fn default() {
    assert_eq!(Foo::answer(), 42);
}

fn main() {
    Foo::answer();
}
