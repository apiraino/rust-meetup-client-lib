use reqwest::{Error as ReqwestError, Method, Url};
use super::response::Response;

#[derive(Debug)]
pub enum HttpClientError {
    InitError(ReqwestError),
    RequestError(ReqwestError),
    SendError(ReqwestError),
}

pub type HttpClientResponse = Result<Box<Response>, HttpClientError>;

pub trait HttpClient {
    fn request(&self, method: Method, url: Url) -> HttpClientResponse;
}
