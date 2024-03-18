#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// we can use enum inside the enum
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//how to get inner value of Option with match
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// we can use _ if there is not other value and () if there is nothing to do
fn dice_result(dice_roll: i32){
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}