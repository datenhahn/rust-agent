#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate subprocess;
extern crate reqwest;

use std::io::prelude::*;
use std::fs::File;
use subprocess::Exec;
use std::io::BufReader;
use reqwest::Url;

#[derive(Deserialize)]
struct Config {
    url: String,
    id: String,
    connector: String,
}

fn main() {
    let mut file = File::open("config.toml").expect("Unable to open the config file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read the file");

    let config: Config = toml::from_str(contents.as_str()).unwrap();

    println!("ID: {}", config.id);
    println!("URL: {}", config.url);
    println!("CONNECTOR: {}", config.connector);

    let stream = Exec::cmd(config.connector).stream_stdout().expect("could not read from stdout");
    let reader = BufReader::new(stream);
    let client = reqwest::Client::new();

    for line in reader.lines() {
        let myline = line.expect("line read failed");
        println!("line: {}", myline);
        let res = client.post(Url::parse(&config.url).unwrap()).body(myline).send();
        println!("result: {}", res.expect("invalid response").status())
    }
}