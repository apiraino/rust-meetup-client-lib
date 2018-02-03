use std::result::Result;
use reqwest::{Error as ReqwestError, Method, Url};
extern crate serde;
extern crate serde_json;

use http_client::http_client::{HttpClient, HttpClientError, HttpClientResponse};
use http_client::http_client_default::DefaultHttpClient;

// ref. https://github.com/rust-on-slack/rust-slack-inviter/blob/master/src/slack.rs

#[derive(Deserialize)]
pub struct MeetupResult {
    pub id: String,
    pub member_id: String,
    pub message_id: Option<String>,
    pub is_reply: Option<String>, // "False",
    pub updated: Option<String>,  // "Sun Nov 19 17:19:59 EST 2017"
    pub discussion_body: Option<String>,
    pub discussion_title: Option<String>,
    pub photo_url: Option<String>, // "https://secure.meetupstatic.com/photos/member/c/8/7/3/thumb_271311315.jpeg",
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub link: Option<String>,
    pub published: Option<String>, // "Sun Nov 19 17:19:59 EST 2017",
    pub title: Option<String>,
    pub member_name: Option<String>,
    pub thread_id: Option<String>,
    pub group_id: Option<String>,
}

#[derive(Deserialize)]
pub struct MeetupActivityResponseSerializer {
    pub results: Vec<MeetupResult>,
}

pub struct MeetupClient {
    // only the last field of a struct may have a dynamically sized type
    // Box allocated on the heap rather than the stack
    // https://rustbyexample.com/std/box.html
    http_client: Box<HttpClient>,
    url: Url,
}

#[derive(Debug)]
pub enum MeetupClientError {
    RequestError(HttpClientError),
    RequestError2(ReqwestError),
}

// https://www.reddit.com/r/rust/comments/69i105/the_grass_is_always_greener_my_struggles_with_rust/dh71fzh/
// https://www.reddit.com/r/rust/comments/69i105/the_grass_is_always_greener_my_struggles_with_rust/dh6ryh0/
type ClientResult<T> = Result<T, MeetupClientError>;

impl MeetupClient {
    pub fn new(&self, http_client: Box<HttpClient>, base_url: &str, token: &str) -> MeetupClient {
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut().append_pair("key", token);
        MeetupClient { http_client, url }
    }

    pub fn create_default(&self, base_url: &str, token: &str) -> MeetupClient {
        let boxed_client: Box<DefaultHttpClient> = Box::new(DefaultHttpClient {});
        self.new(boxed_client, base_url, token)
    }

    pub fn get_activity(&self, member_id: &str) -> ClientResult<MeetupActivityResponseSerializer> {
        let mut _url = self.url.clone();
        _url.query_pairs_mut().append_pair("member_id", member_id);
        _url.set_path("activity");
        let resp = self._make_request(Method::Get, _url);
        // debug serialize and print (need to cast)
        // println!("{:?}", resp);
        // let resp_data: MeetupActivityResponseSerializer = resp.json().unwrap();
        // println!("{:?}", resp_data);
        resp.map_err(MeetupClientError::RequestError)?
            .json()
            .map_err(MeetupClientError::RequestError2)
    }

    fn _make_request(&self, method: Method, url: Url) -> HttpClientResponse {
        self.http_client.request(method, url)
    }
}
