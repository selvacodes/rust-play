// Rust 1.19, Hyper 0.11, tokio-core 0.1, futures 0.1

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use futures::{Future, Stream};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    current: Current,
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    condition: Condition,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    text: String,
    code: i32,
}

fn main() {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let url: Uri = "http://api.apixu.com/v1/current.json?key=8e8bca5064f849fe87c102259173008&q=Paris"
        .parse()
        .unwrap();

    let request = client
        .get(url)
        .and_then(|res| res.body().concat2())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
        .and_then(|body| {
            serde_json::from_slice(&body).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err)
            })
        });

    match core.run(request) {
        Ok(Weather { current }) => println!("{:?}", current),
        Err(err) => println!("{}", err),
    }
}
