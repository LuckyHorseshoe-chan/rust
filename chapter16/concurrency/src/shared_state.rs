use std::sync::{Arc, Mutex};
use std::thread;

// use Mutex to allow access to data from one thread at a time
// use an Arc<T> instead Rc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}