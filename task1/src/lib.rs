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

pub fn print_post(post: &Post){
    println!("Post {}", post.id);
    println!("{}, {}, {}", post.title, post.author, post.date);
    println!("{}", post.content);
    println!("\n");
}

impl Blog {
    pub fn get_all(&self) {
        let posts_iter = self.posts.iter();
        for post in posts_iter {
            print_post(post);
        }
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn get_post_by_id(&self, id: u64) {
        let posts_iter = self.posts.iter();
        for post in posts_iter {
            if post.id == id {
                print_post(post);
                return;
            }
        }
        println!("Not found");
    }
}