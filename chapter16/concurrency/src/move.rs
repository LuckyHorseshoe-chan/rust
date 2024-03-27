use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // write move to use a variable from the global enviroment
    // it moves to the spawn enviroment
    // without move Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}