use meetup_client::MeetupActivityResponseSerializer;
use super::response::Response as ResponseInterface;
use reqwest::Response;
use reqwest::Result;

pub struct DefaultResponse {
    response: Response
}

impl DefaultResponse {
    pub fn new(response: Response) -> Self {
        DefaultResponse{ response }
    }
}

impl ResponseInterface for DefaultResponse {
    fn json(&mut self) -> Result<MeetupActivityResponseSerializer> {
        self.response.json()
    }
}