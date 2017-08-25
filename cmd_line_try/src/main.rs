extern crate clap;
//extern crate futures;
//extern crate hyper;
//extern crate tokio_core;

use clap::{App, Arg};
//use std::str::FromStr;
//use futures::{Future, Stream};
//use hyper::{Client, Response, StatusCode, Uri};
//use tokio_core::reactor::Core;

fn main() {
    let matches = App::new("htm")
        .version("1.0")
        .author("selvacodes")
        .about("reads html from link")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("LINK")
                .help("Sets URL")
                .takes_value(true),
        )
        .get_matches();

    let url = matches.value_of("url");

    match url {
        Some(val) => println!("value {}", val),
        None => println!("invalid"),
    };
}

//fn make_request(url: &str) -> Future<Item = Response, Error = std::error::Error> {
//let mut core = Core::new()?;
//let client = Client::new(&core.handle());

//let uri = "http://httpbin.org/ip".parse()?;
//let work = client.get(uri).and_then(|res| {
//println!("Response: {}", res.status());

//res.body().for_each(|chunk| {
//io::stdout().write_all(&chunk).map_err(From::from)
//})
//});
//core.run(work)?;
//work
//}
