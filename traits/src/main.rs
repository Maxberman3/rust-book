mod aggregator;

use aggregator::{Summary, Tweet, Book, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let book = Book {
        title: String::from("The Book"),
        author: String::from("The Author"),
        year: 2020,
    };

    println!("Book: {}", book.summarize());

    println!("Notify: ");
    notify(&tweet);
}