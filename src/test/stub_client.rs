use http_client::{HttpClient, HttpClientResponse, Response};
use reqwest::{Method, Url};

pub struct StubClient {
    response: Box<Response>
}

impl StubClient {
    pub fn new(response: Box<Response>) -> Self {
        StubClient{ response }
    }
}

impl HttpClient for StubClient {
    fn request(&self, _method: Method, _url: Url) -> HttpClientResponse {
        Ok(self.response)
    }
}