use reqwest::Result;
use meetup_client::MeetupActivityResponseSerializer;

pub trait Response {
    fn json(&mut self) -> Result<MeetupActivityResponseSerializer>;
}