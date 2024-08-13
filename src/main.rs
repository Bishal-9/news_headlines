
mod theme;

use dotenv::dotenv;
use std::error::Error;
use newsapi::{Article, Country::IN, Endpoint, NewsAPI};

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines \n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.get_title()));
        theme.print_text(&format!("> *{}*", i.get_url()));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    dotenv();

    let api_key = std::env::var("API_KEY")?;

    let mut news_api_instance = NewsAPI::new(&api_key);
    news_api_instance
        .country(IN)
        .endpoint(Endpoint::TopHeadlines);
    
    let top_headlines_response = news_api_instance.blocking_fetch()?;

    render_articles(&top_headlines_response.get_articles());

    Ok(())
}
