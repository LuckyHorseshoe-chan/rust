use std::thread;

fn immutable() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// between the closure definition and the closure call, an immutable borrow to print isn’t allowed 
// because no other borrows are allowed when there’s a mutable borrow
fn mutable() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

//  even though the closure body still only needs an immutable reference, we need to specify that list 
// should be moved into the closure by putting the move keyword at the beginning of the closure definition
fn spawn(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}