use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }


}

// Defining a Trait, trait is similar to an interface in other languages
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Trait bounds
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds with where clause
pub fn notify_with_where<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Return types the implement the Summary trait
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("John Doe"),
        content: String::from("This is a tweet"),
        reply: false,
        retweet: false,
    }
}

// Conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

fn main() {
    let new_article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Breaking News"),
        content: String::from("This is a breaking news"),
    };

    let tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("This is a tweet"),
        reply: false,
        retweet: false,
    };

    println!("New Article: {}", new_article.summarize());
    println!("Tweet: {}", tweet.summarize_author());
    
    notify(&new_article);

    notify_with_where(&tweet);

    let item = returns_summarizable();
    println!("Item: {}", item.summarize());

    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let pair = Pair::new("John", "Doe");
    pair.cmp_display();
}
