let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}
fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
}

// type alias Thunk
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = Box::new(|| println!("hi"));
fn takes_long_type(f: Thunk) {
    // --snip--
}
fn returns_long_type() -> Thunk {
    // --snip--
}

// never type (empty type)
fn bar() -> ! {
    // --snip--
}
// continue is !, so it can be converted to any type (u32 in this case)
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
// error: guess must have only one type
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};

// Rust provides the Sized trait to determine whether or not a type’s size is known at compile time
// a trait bound on ?Sized means “T may or may not be Sized”
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}