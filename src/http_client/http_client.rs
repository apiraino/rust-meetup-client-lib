extern crate reqwest;
use reqwest::{Error as ReqwestError, Method, Response, Url};

#[derive(Debug)]
pub enum HttpClientError {
    InitError(ReqwestError),
    RequestError(ReqwestError),
    SendError(ReqwestError),
}

pub type HttpClientResponse = Result<Response, HttpClientError>;

pub trait HttpClient {
    fn request(&self, method: Method, url: Url) -> HttpClientResponse;
}
