use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) ->  String {
                format!("(read more from {}...)", self.summarize_author())
        }
}

// 摘要
// 内容
pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
}

impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
                format!("@{}", self.author)
        }
}


pub struct Weibo {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
}

impl Summary for Weibo {
        fn summarize_author(&self) -> String {
                format!("@{}", self.username)
        }
}

pub fn notify(item: impl Summary) {
        println!("Breaking News: {}", item.summarize());
}

pub fn notify0(item1: impl Summary, item2: impl Summary) {
        println!("Breaking News: {} {}", item1.summarize(), item2.summarize());
}

pub fn notify1<T: Summary>(item1: T, item2: T) {
        println!("Breaking News: {} {}", item1.summarize(), item2.summarize());
}

pub fn notify2(item1: impl Summary + Display) {
        println!("Breaking News: {}", item1.summarize());
}

pub fn notify3<T: Summary + Display>(item1: T) {
        println!("Breaking News: {}", item1.summarize());
}

pub fn notify_x<T: Summary + Display, U: Debug + Clone>(a: T, b: U) -> String {
        format!("Breaking News: {}", a.summarize())
}

pub fn notify_y<T, U>(a: T, b: U) -> String 
        where T: Summary + Display,
              U: Clone + Debug, 
{
        format!("Breaking News: {}", a.summarize())
}

pub fn notify_z(flag: bool) -> impl Summary {

        if flag {
                NewsArticle {
                        headline: String::from("Breaking News"),
                        content: String::from("content 内容随便写写好了"),
                        author: String::from("CodeCore"),
                        location: String::from("China, Shanghai"),
                }
        } else {
                // Weibo {
                //         username: String::from("CodeCore"),
                //         reply: true,
                //         retweet: true,
                //         content: String::from("content 内容随便"),
                // }

                NewsArticle {
                        headline: String::from("NewsArticle second"),
                        content: String::from("content 内容随便写写好了 😄"),
                        author: String::from("Gooooonix"),
                        location: String::from("China, Shanghai"),
                }
        }
        
}


