/** 10_2_traits.rs
 *
 * Topic: traits in Rust
 * Tells the Rust compiler about functionality a particular type has
 * and can share with other types
 * example: media aggregator library
 *
 * Xuhua Huang
 * Created: Aug 1, 2021
 * Last updated: Aug 2, 2021
 */
use learn_10_2_traits::{NewsArticle, Summary, Tweet};

fn main() {
    println!(); // added this python style of new line
    println!("Let's talk about Traits - Shared Behaviours in Rust!");

    /* Using NewsArticle and Tweet struct with implemented trait */
    let my_tweet: Tweet = Tweet {
        username: String::from("Xuhua Huang"),
        content: String::from("Of course, you probably already know"),
        reply: false,
        retweet: false,
    };
    println!("\nObject `my_tweet` summary: {}", my_tweet.summarize());

    /* Using NewsArticle and NewsArticle struct with implemented trait */
    /* Also with default implementation */
    let new_article: NewsArticle = NewsArticle {
        headline: String::from("China just earned its 60th Olympic metal in Tokyo 2020!"),
        location: String::from("Tokyo, Japan"),
        author: String::from("Xuhua Huang"),
        content: String::from(
            "The team China is the best performing team in terms of earning gold metal! \
             Let's congregate China for its well performing!",
        ),
    };
    println!("New article available! {}", new_article.summarize());
}
