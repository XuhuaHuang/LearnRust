/**
 * lib.rs
 * media aggregator library
 * 
 * Xuhua Huang
 * Created: Aug 1, 2021
 * Last updated: Aug 2, 2021
 */

/* Provide  Signature for a Commonly Implemented Method */
pub trait Summary {
    /** This trait work with different type of media content
     * instead of writing a complete implementation, only the method signature is provided
     * compiler will enforce type that has the `Summary` trait implement the `summarize` method
     * think of this as the interface concept in C++ or Java
     */
    fn summarize(&self) -> String {
        /* Provide default implementation */
        format!("(Read more from author {}...)", self.summarize_author())
    }

    /* Add a summarize_author method to the trait */
    fn summarize_author(&self) -> String;
}

/* Implementing Summary::summarize on `NewsArticle` Type */
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} by ({})", self.headline, self.location, self.author)
    }
    /* Add summary_author method for the NewsArticle struct */
    fn summarize_author(&self) -> String {
        format!("Read more from {}", self.author)
    }
}

/* Implementing Summary::summarize on `Tweet` Type */
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    /* Add summarize_author for struct Tweet */
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/* Parse Trait as parameters */
/* using trait bound syntax */
// pub fn notify<T: Summary>(item: &T)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* Specifying multiple trait bounds with the + syntax */
// pub fn notify(item: &(impl Summary + Display)) {}

/* Returning Types that Implements Traits */
fn returns_summarized() -> impl Summary {
    Tweet {
        username: String::from("Xuhua Huang"),
        content: String::from(
            "China has been doing great in Tokyo 2020 Olympic!"
        ),
        reply: false,
        retweet: false,
    }
}

// TODO: https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits