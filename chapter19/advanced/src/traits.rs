// associated types
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// A baby dog is called a Spot
println!("A baby dog is called a {}", Dog::baby_name());
// A baby dog is called a puppy
println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

// supertraits
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point {
    x: i32,
    y: i32,
}
impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype pattern
// creating a Wrapper type around Vec<String> to implement Display
use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
let w = Wrapper(vec![String::from("hello"), String::from("world")]);
println!("w = {}", w);