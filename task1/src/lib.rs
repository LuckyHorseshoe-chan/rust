use itertools::Itertools;

pub struct Post {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String
}
pub struct Blog {
    pub posts: Vec<Post>
}

impl std::fmt::Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Post {}\n{}, {}, {}\n{}\n", self.id, self.title, self.author, self.date, self.content)
    }
}

impl std::fmt::Display for Blog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let posts_str: String = self.posts.iter().map(|post| format!("{}", post)).join("");
        write!(f, "{}", posts_str)
    }
}

impl Blog {
    pub fn get_all(&self) -> String {
        return self.to_string();
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn get_post_by_id(&self, id: u64) -> String {
        let posts_iter = self.posts.iter();
        for post in posts_iter {
            if post.id == id {
                return post.to_string();
            }
        }
        return String::from("Not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all_posts() {
        let blog = Blog {
            posts: vec![
                Post {
                    id: 1,
                    title: String::from("Using structs"),
                    author: String::from("Jane Doe"),
                    date: String::from("12-04-2020"),
                    content: String::from("Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.")
                },
                Post {
                    id: 2,
                    title: String::from("Recoverable Errors with Result"),
                    author: String::from("Mei Gyn"),
                    date: String::from("30-08-2022"),
                    content: String::from("Most errors aren’t serious enough to require the program to stop entirely.")
                }
            ]
        };

        assert_eq!(
            blog.get_all(),
            "Post 1\nUsing structs, Jane Doe, 12-04-2020\nStructs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.\nPost 2\nRecoverable Errors with Result, Mei Gyn, 30-08-2022\nMost errors aren’t serious enough to require the program to stop entirely.\n"
        );
    }

    #[test]
    fn add_post() {
        let mut blog = Blog {
            posts: vec![
                Post {
                    id: 1,
                    title: String::from("Using structs"),
                    author: String::from("Jane Doe"),
                    date: String::from("12-04-2020"),
                    content: String::from("Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.")
                },
                Post {
                    id: 2,
                    title: String::from("Recoverable Errors with Result"),
                    author: String::from("Mei Gyn"),
                    date: String::from("30-08-2022"),
                    content: String::from("Most errors aren’t serious enough to require the program to stop entirely.")
                }
            ]
        };

        let p3 = Post {
            id: 3,
            title: String::from("Generic Data Types"),
            author: String::from("Olly Schwarz"),
            date: String::from("05-09-2023"),
            content: String::from("We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.")
        };

        blog.add_post(p3);

        assert_eq!(
            blog.get_all(),
            "Post 1\nUsing structs, Jane Doe, 12-04-2020\nStructs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.\nPost 2\nRecoverable Errors with Result, Mei Gyn, 30-08-2022\nMost errors aren’t serious enough to require the program to stop entirely.\nPost 3\nGeneric Data Types, Olly Schwarz, 05-09-2023\nWe use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.\n"
        );
    }

    #[test]
    fn get_post_by_id() {
        let blog = Blog {
            posts: vec![
                Post {
                    id: 1,
                    title: String::from("Using structs"),
                    author: String::from("Jane Doe"),
                    date: String::from("12-04-2020"),
                    content: String::from("Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.")
                },
                Post {
                    id: 2,
                    title: String::from("Recoverable Errors with Result"),
                    author: String::from("Mei Gyn"),
                    date: String::from("30-08-2022"),
                    content: String::from("Most errors aren’t serious enough to require the program to stop entirely.")
                },
                Post {
                    id: 3,
                    title: String::from("Generic Data Types"),
                    author: String::from("Olly Schwarz"),
                    date: String::from("05-09-2023"),
                    content: String::from("We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.")
                }
            ]
        };

        assert_eq!(
            blog.get_post_by_id(2),
            "Post 2\nRecoverable Errors with Result, Mei Gyn, 30-08-2022\nMost errors aren’t serious enough to require the program to stop entirely.\n"
        )
    }

    #[test]
    fn get_not_found() {
        let blog = Blog {
            posts: vec![
                Post {
                    id: 1,
                    title: String::from("Using structs"),
                    author: String::from("Jane Doe"),
                    date: String::from("12-04-2020"),
                    content: String::from("Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.")
                },
                Post {
                    id: 2,
                    title: String::from("Recoverable Errors with Result"),
                    author: String::from("Mei Gyn"),
                    date: String::from("30-08-2022"),
                    content: String::from("Most errors aren’t serious enough to require the program to stop entirely.")
                },
                Post {
                    id: 3,
                    title: String::from("Generic Data Types"),
                    author: String::from("Olly Schwarz"),
                    date: String::from("05-09-2023"),
                    content: String::from("We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.")
                }
            ]
        };

        assert_eq!(
            blog.get_post_by_id(4),
            "Not found"
        )
    }
}