#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // the reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    // error: this closure can be called once; trying to call it a second time wouldn’t work because value would no longer be in the environment to be pushed into sort_operations again!
    // the closure must implement FnMut, not FnOnce
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);

    // using an FnMut closure with sort_by_key is allowed
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}