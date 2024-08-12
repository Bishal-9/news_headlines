
use colour::{dark_green, yellow};
use std::error::Error;
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://newsapi.org/v2/top-headlines?country=in&apiKey=364a7519b0614dd8b15fbb8a774b6b66";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
