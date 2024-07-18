use reqwest::{header, Response, Error};
use serde::{Deserialize, Serialize};

pub struct HttpClient {
    base_url: String,
    headers: header::HeaderMap,
   pub client: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: &str, headers: header::HeaderMap) -> Result<Self, Error> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(Self {
            base_url: base_url.to_owned(),
            headers,
            client,
        })
    }
  pub async fn get(&self, path: &str) -> Result<String, Error>
       {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).headers(self.headers.clone()).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn get_json<T>(&self, path: &str) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).headers(self.headers.clone()).send().await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn post<T>(&self, path: &str, body: &T) -> Result<Response, Error>
    where
        T: Serialize + std::fmt::Debug,
    {

        let url = format!("{}{}", self.base_url, path);
        let resp = self.client
            .post(&url)
            .headers(self.headers.clone())
            .json(body)
            .send()
            .await?;

        Ok(resp)
    }
}

