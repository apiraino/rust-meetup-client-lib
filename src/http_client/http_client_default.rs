use reqwest::{Client, Method, Url, Response};

use http_client::{HttpClient, HttpClientError, HttpClientResponse, Response as ResponseInterface, DefaultResponse};

pub struct DefaultHttpClient {}

impl DefaultHttpClient {
    fn wrap_response(response: Response) -> Box<ResponseInterface> {
        Box::new(DefaultResponse::new(response)) as Box<ResponseInterface>
    }
}

impl HttpClient for DefaultHttpClient {
    fn request(&self, method: Method, url: Url) -> HttpClientResponse {
        // which error raised?
        let client = Client::new().map_err(HttpClientError::InitError)?;
        client
            .request(method, url)
            .map_err(HttpClientError::RequestError)?
            .send()
            .map_err(HttpClientError::SendError)
            .map(DefaultHttpClient::wrap_response)
    }
}
