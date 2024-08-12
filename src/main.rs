
use colour::{dark_green, yellow};
use dotenv::dotenv;
use std::error::Error;
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url = format!("https://newsapi.org/v2/top-headlines?country=in&apiKey={}", api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
