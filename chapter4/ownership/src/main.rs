fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let x = 5;
    let y = x;

    println!("y: {y}"); // it works, the deep copy is used with numbers, numbers are stored in the stack

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1); // error, the pointer moved from s1 to s2 (shallow copy), strings are stored in the heap, pointers - in the stack

    let a1 = String::from("hello");
    let a2 = a1.clone(); // deep copy

    println!("s1 = {}, s2 = {}", a1, a2); // that's works

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("it's okay to still use x: {x}");
    
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                    // takes_and_gives_back, which also
                                    // moves its return value into s3
    let h1 = String::from("hello");

    let (h2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", h2, len);

    let mut s = String::from("hello");

    // At any given time, you can have either one mutable reference or any number of immutable references
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
                                
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                           // return value into the function
                                           // that calls it

  let some_string = String::from("yours"); // some_string comes into scope

  some_string                              // some_string is returned and
                                           // moves out to the calling
                                           // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                    // scope

  a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}

// fn dangle() -> &String { // dangle returns a reference to a String

//   let s = String::from("hello"); // s is a new String

//   &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!
// // The solution here is to return the String directly
fn no_dangle() -> String {
  let s = String::from("hello");

  s
}

// str is a string slice type, &str allows to use both &String and &str
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}