use std::io;

fn main() {
    println!("Write a temperature in Celsius");
    let mut cel = String::new();
    io::stdin()
        .read_line(&mut cel)
        .expect("Failed to read line");

    let cel: f32 = cel
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let far = fahrenheit(cel);
    println!("A temperature in Fahrenheit: {far}");
    println!("Write a Fibonacci index");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let fib = fibonacci_num(n);
    println!("fibonacci_num: {fib}");
    christmas_carol();
}

fn fibonacci_num(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut f0, mut f1) = (0, 1);
    for f in 2..n+1 {
        let tmp = f1;
        f1 = f0 + f1;
        f0 = tmp;
    }
    return f1;
}
fn fahrenheit(cel: f32) -> f32 { (cel * 9.0 / 5.0) + 32.0 }
fn christmas_carol() {
    println!("The Twelve Days of Christmas:");
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "nineth", 
        "tenth", 
        "eleventh", 
        "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..12 {
        let day = days[i];
        println!("On the {day} day of Christmas, my true love sent to me");
        for j in (0..i+1).rev() {
            println!("{}", gifts[j]);
        }
    }
}