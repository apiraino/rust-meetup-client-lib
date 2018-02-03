extern crate reqwest;
use reqwest::{Client, Method, Url};

use http_client::http_client::{HttpClient, HttpClientError, HttpClientResponse};

pub struct DefaultHttpClient {}

impl HttpClient for DefaultHttpClient {
    fn request(&self, method: Method, url: Url) -> HttpClientResponse {
        // which error raised?
        let client = Client::new().map_err(HttpClientError::InitError)?;
        client
            .request(method, url)
            .map_err(HttpClientError::RequestError)?
            .send()
            .map_err(HttpClientError::SendError)
    }
}
