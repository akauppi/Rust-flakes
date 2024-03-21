
use rand;

#[derive(Copy, Clone)]
enum Foo {
    Bar = 1,
    Car = 2
}

fn random_enum() -> Foo {
    let b = rand::random::<bool>();

    if b { Foo::Bar } else { Foo::Car }
}

fn main() {
    let a = random_enum();
    assert_eq!(a as u8, 1_u8);

    println!("Hello, world! {}", a as u8);
}
