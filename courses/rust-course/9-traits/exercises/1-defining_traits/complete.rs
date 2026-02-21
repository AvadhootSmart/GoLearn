// 1. Define Summary trait
pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
}

// 2. Implement Summary for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Article summary")
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.0 Released"),
    };
    
    // 3. Print the summary of the article
    println!("{}", article.summarize());
}
