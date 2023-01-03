use hello_trait::Summary;
use hello_trait::NewsArticle;



fn main() {
        let na = NewsArticle {
                headline: String::from("Hello headling"),
                content: String::from("jkfad;gjask;gj;dslkajgdl;skagjds;lkahjdsklah😊"),
                author: String::from("John Doe"),
                location: String::from("ShangHai"),

        };

        println!("1 weibo reply: {}", na.summarize());
}
