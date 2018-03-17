#[cfg(test)]
extern crate reqwest;
extern crate serde;

mod stub_client;
mod stub_response;

use meetup_client::{MeetupClient, MeetupResult, MeetupActivityResponseSerializer};
use self::stub_client::StubClient;
use self::stub_response::StubResponse;

fn test_smt() {
    let result = MeetupResult{
        id: "id".to_string(),
        member_id: "member_id".to_string(),
        message_id: Some("message_id".to_string()),
        is_reply: Some("is_reply".to_string()),
        updated: Some("updated".to_string()),
        discussion_body: Some("discussion_body".to_string()),
        discussion_title: Some("discussion_title".to_string()),
        photo_url: Some("photo_url".to_string()),
        group_name: Some("group_name".to_string()),
        item_type: Some("item_type".to_string()),
        link: Some("link".to_string()),
        published: Some("published".to_string()),
        title: Some("title".to_string()),
        member_name: Some("member_name".to_string()),
        thread_id: Some("thread_id".to_string()),
        group_id: Some("group_id".to_string())
    };
    let results = vec![result];
    let serializer = MeetupActivityResponseSerializer { results };
    let response = Box::new(StubResponse::new(serializer));
    let http_client = Box::new(StubClient::new(response));
    let base_url = "test-url";
    let token = "test-token";
    let client = MeetupClient::new(http_client, base_url, token);
}