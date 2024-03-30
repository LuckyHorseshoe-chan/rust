fn main() {
    // irrefutable pattern (x matches anything and therefore cannot fail to match)
    let x = 5;
    // refutable pattern (if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match)
    //if let Some(x) = a_value;

    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // inner y isn't the outer y, it's shadowed
    // so y = 10 at the end of the program
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // multiple patterns: | is OR
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    // also works with chars
    let x = 5;
    match x {
        // it's the same as 1 | 2 | 3 | 4 | 5
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // destructuring a struct's fields
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // alternative let Point { x, y } = p; - saves to x and y variables
    assert_eq!(0, a);
    assert_eq!(7, b);

    // match separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither
    // remember that a match expression stops checking arms once it has found the first matching pattern, 
    // so even though Point { x: 0, y: 0} is on the x axis and the y axis, this code would only print On the x axis at 0
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // destructuring enum variants that hold different kinds of values
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignore value with the _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // there won't be warning about not used x because we wrote _
    let _x = 5;
    let y = 10;

    // error: s moved to _s
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // works fine because we never bind s to anything
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // use .. to ignore remaining parts of value
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    // but it should be unambiguous
    // this code will give error
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }

    // match guards
    // the outer y isn't shadowed anymore
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // bindings
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

}
