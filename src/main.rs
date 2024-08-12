
use colour::{dark_green, yellow};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Articles  {
    articles: Vec<Article>
}

#[derive(Debug, Deserialize)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

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
