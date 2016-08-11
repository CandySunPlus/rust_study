fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}


trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Baz's impl of Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Baz's impl of Bar");
    }
}

fn main() {
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);

    let b = Baz;
    <Baz as Bar>::f(&b);
    Foo::f(&b);
}
