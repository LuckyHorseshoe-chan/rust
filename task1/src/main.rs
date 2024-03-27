use task1::Blog;
use task1::Post;

fn main() {
    let mut blog = Blog {
        posts: Vec::new()
    };
    let p1 = Post {
        id: 1,
        title: String::from("Using structs"),
        author: String::from("Jane Doe"),
        date: String::from("12-04-2020"),
        content: String::from("Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.")
    };
    let p2 = Post {
        id: 2,
        title: String::from("Recoverable Errors with Result"),
        author: String::from("Mei Gyn"),
        date: String::from("30-08-2022"),
        content: String::from("Most errors aren’t serious enough to require the program to stop entirely.")
    };
    let p3 = Post {
        id: 3,
        title: String::from("Generic Data Types"),
        author: String::from("Olly Schwarz"),
        date: String::from("05-09-2023"),
        content: String::from("We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.")
    };
    blog.add_post(p1);
    blog.add_post(p2);
    blog.add_post(p3);
    println!("get all posts:");
    println!("{}", blog.get_all());
    println!("get post 2:");
    blog.get_post_by_id(2);
    println!("get post 4:");
    blog.get_post_by_id(4);
}
