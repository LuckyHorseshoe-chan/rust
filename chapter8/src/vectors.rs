fn vec_main() {
    // create a new empty vector
    let v: Vec<i32> = Vec::new();

    // because we’ve given initial i32 values, the type annotation isn’t necessary
    let v = vec![1, 2, 3];

    // update vector
    let mut v = Vec::new();
    v.push(5);

    // two ways of indexing
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // panic
    let does_not_exist = &v[100];
    // None
    let does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // vectors can only store values that are the same type
    // use enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}