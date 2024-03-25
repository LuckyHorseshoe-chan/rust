use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // using the dereference operator to follow a reference to an i32 value
    assert_eq!(5, *y);

    // using a Box<T> instead of a reference
    // we set y to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // with deref
    hello(&m);
    // without deref
    hello(&(*m)[..]);
}