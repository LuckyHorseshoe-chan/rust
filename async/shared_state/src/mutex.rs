use std::sync::{Mutex, MutexGuard};

// This code doesn't work because the std::sync::MutexGuard type is not Send
// You can't send a mutex lock to another thread, and the error happens because the Tokio runtime can move a task between threads at every .await
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here

// To avoid this, you should restructure your code such that the mutex lock's destructor runs before the .await
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}

// Also you can wrap the mutex in a struct, and only ever lock the mutex inside non-async methods on that struct
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

// The tokio::sync::Mutex type provided by Tokio can also be used. 
// The primary feature of the Tokio mutex is that it can be held across an .await without any issues. 
// That said, an asynchronous mutex is more expensive than an ordinary mutex, and it is typically better to use one of the two other approaches.

use tokio::sync::Mutex; // note! This uses the Tokio mutex

// This compiles!
// (but restructuring the code would be better in this case)
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here