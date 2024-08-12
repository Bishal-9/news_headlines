
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Articles  {
    pub articles: Vec<Article>
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}
