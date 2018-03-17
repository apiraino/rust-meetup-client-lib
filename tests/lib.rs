extern crate meetup_client_lib;
use std::env;
use meetup_client_lib::MEETUP_URL;
use meetup_client_lib::meetup_client::MeetupClient;

fn setup() -> MeetupClient {
    let token: &str = &env::var("MEETUP_API_KEY")
        .expect("MEETUP_API_KEY was not found.")
        .to_string();
//    MeetupClient::new(MEETUP_URL, token)
}
fn teardown() {
    // undo what you've done in setup()
}

#[test]
fn test_activity() {
    let client = setup();
    let member_id: &str = &env::var("MEMBER_ID").expect("MEMBER_ID was not found.");
    let resp_data = client.get_activity(member_id);
    let results_data = resp_data.unwrap();
    let results = results_data.results;
    for result in results {
        assert_ne!("", result.title.as_ref().unwrap());
        assert_ne!("", result.member_name.as_ref().unwrap());
        assert_ne!("", result.id);
    }
}
