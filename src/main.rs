
mod theme;

use dotenv::dotenv;
use std::error::Error;
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top Headlines \n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("---");
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
