extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
pub mod meetup_client;
pub mod http_client;

pub static MEETUP_URL: &str = "https://api.meetup.com";

// quick_error! {
//     #[derive(Debug)]
//     pub enum MyError {
//         ReqwestError(err: reqwest::Error) {
//             description(err.description())
//         }
//         ClientError(err: HttpClientError) {
//             description(err.description())
//         }
//     }
// }
