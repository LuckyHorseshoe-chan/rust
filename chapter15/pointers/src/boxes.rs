enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// if use List the compiler will think that there is infinite memory size and the error occurs
// if we use Box, the memory size of a box’s pointer data is requested
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}