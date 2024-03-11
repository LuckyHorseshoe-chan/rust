fn main() {
    print_labeled_measurement(5, 'h');
    another_function(5);
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// everything is awesome while you don't paste ; after x + 1
fn plus_one(x: i32) -> i32 {
    x + 1
}