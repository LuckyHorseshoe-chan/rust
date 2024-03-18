// Rust hasn't type null, so the enum Option<T> is used
enum Option<T> {
    None,
    Some(T),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// enum is more convineint than 4 structs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // error because y can be None

}
