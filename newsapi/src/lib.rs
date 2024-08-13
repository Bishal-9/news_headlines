
#[cfg(feature = "async")]
use reqwest::{Client, Method};
use serde::Deserialize;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {

    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),

    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),

    #[error("Articles parsing failed")]
    ArticlesParseFailed(serde_json::Error),

    #[error("Request failed: {0}")]
    BadRequest(&'static str),

    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error)
}

#[derive(Debug, Deserialize)]
pub struct NewsApiResponse {
    status: String,
    articles: Vec<Article>,
    code: Option<String>
}

impl NewsApiResponse {
    pub fn get_articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Debug, Deserialize)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

pub enum Endpoint {
    Business,
    Crypto,
    Everything,
    General,
    Health,
    Science,
    Sports,
    Technology,
    TopHeadlines
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::Business => "/top-headlines?category=business&pageSize=100".to_string(),
            Self::Crypto => "/top-headlines?q=crypto&pageSize=100".to_string(),
            Self::Everything=> "/everything?pageSize=100".to_string(),
            Self::General => "/top-headlines?category=general&pageSize=100".to_string(),
            Self::Health => "/top-headlines?category=health&pageSize=100".to_string(),
            Self::Science => "/top-headlines?category=science&pageSize=100".to_string(),
            Self::Sports => "/top-headlines?category=sports&pageSize=100".to_string(),
            Self::Technology => "/top-headlines?category=technology&pageSize=100".to_string(),
            Self::TopHeadlines => "/top-headlines?pageSize=100".to_string(),
        }
    }
}

pub enum Country {
    IN,
    GB,
    USA
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::GB => "gb".to_string(),
            Self::IN => "in".to_string(),
            Self::USA => "us".to_string()
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country
}

impl NewsAPI {

    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            country: Country::IN,
            endpoint: Endpoint::Everything
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;

        self
    }
    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;

        self
    }
    fn prepare_url(&self) -> Result<String, NewsApiError> {        
        let mut url = format!("{}{}&country={}", BASE_URL, &self.endpoint.to_string(), &self.country.to_string());

        Ok(url)
    }
    pub fn blocking_fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let request = ureq::get(&url)
            .set("X-Api-Key", self.api_key.as_str());
        let response: NewsApiResponse = request.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code))
        }
    }
    #[cfg(feature = "async")]
    pub async fn non_blocking_fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        println!("URL: {}", url);
        let client = Client::new();
        let request = client
            .request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .header("User-Agent", "NewsAPI")
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;
        let response: NewsApiResponse = client.execute(request).await?.json().await.map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code))
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {

    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled."),
            "apiKeyExhausted" => NewsApiError::BadRequest("Your API key has no more requests available."),
            "apiKeyInvalid" => NewsApiError::BadRequest("Your API key hasn't been entered correctly. Double check it and try again."),
            "apiKeyMissing" => NewsApiError::BadRequest("Your API key is missing from the request."),
            "parameterInvalid" => NewsApiError::BadRequest("You've included a parameter in your request which is currently not supported."),
            "parametersMissing" => NewsApiError::BadRequest("Required parameters are missing from the request and it cannot be completed."),
            "rateLimited" => NewsApiError::BadRequest("You have been rate limited. Back off for a while before trying the request again."),
            "sourcesTooMany" => NewsApiError::BadRequest("You have requested too many sources in a single request. Try splitting the request into 2 smaller requests."),
            "sourceDoesNotExist" => NewsApiError::BadRequest("You have requested a source which does not exist."),
            "unexpectedError" => NewsApiError::BadRequest("This shouldn't happen, and if it does then it's our fault, not yours. Try the request again shortly."),
            _ => NewsApiError::BadRequest("Unknown Error")
        }
    } else {
        NewsApiError::BadRequest("Unknown Error")
    }
}
