use meetup_client::MeetupActivityResponseSerializer;
use http_client::Response;
use reqwest::Result;

pub struct StubResponse {
    serializer: MeetupActivityResponseSerializer
}

impl StubResponse {
    pub fn new(serializer: MeetupActivityResponseSerializer) -> Self {
        StubResponse{ serializer }
    }
}

impl Response for StubResponse {
    fn json(&mut self) -> Result<MeetupActivityResponseSerializer> {
        Ok(self.serializer)
    }
}