use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

// we can specify actions when the CustomSmartPointer variable goes out of scope
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // error: explicit destructor calls not allowed
    // c.drop();
    // instead we should use std::mem::drop
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}