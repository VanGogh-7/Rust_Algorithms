enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );
    let m = Box::new(String::from("Rust"));
    hello(&(*m)[..]);

}
