use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        // read line into guess
        // catch possible errors with expect (crash the program)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim removes spaces
        // convert guess to unsigned 32-bit number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break; 
            },
        }
    }
}