fn str_main() {
    // ways to initialize string from the value
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // append to a string
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //  there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters
    // so indexing &hello[0] will return 208, not З as you want
    // you can do slices but remember that one character is 2 bytes
    let hello = "Здравствуйте";
    // Зд
    let s = &hello[0..4];

    // you can iterate over chars or bytes
    // getting grapheme clusters from strings as with the Devanagari script is complex, so this functionality is not provided by the standard library
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }    
}