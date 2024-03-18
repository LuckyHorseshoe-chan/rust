//mod enums;
mod matches;
mod let_if;

fn main() {
    //enums::enum_main();
    let five = Some(5);
    let six = matches::plus_one(five);
    let none = matches::plus_one(None);

    matches::value_in_cents(matches::Coin::Quarter(matches::UsState::Alaska));
    let_if::let_if_main();
}