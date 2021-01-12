use serde::{Deserialize, Serialize};

use sha2::{Sha256, Digest};

use std::fs;
use std::net::{TcpStream};
use std::io::{Write, BufReader, BufRead};

use sha2::digest::Update;

use telegram_bot::*;
use tokio::stream::StreamExt;

#[derive(Deserialize)]
struct Settings {
    token: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // read the config file
    let file = std::fs::File::open("config.json")
        .expect("Cannot create the file");
    let reader = BufReader::new(file);
    let settings: Settings = serde_json::from_reader(reader) // read json
        .expect("There was an error when unwrapping settings file");
    run(settings).await; // pass ownership to run
}

/// Runs the bot instance
async fn run(settings: Settings) -> Result<(), Error> {
    // at first read the config file
    let hashed = base64::decode(settings.password);
    if hashed.is_err() {
        println!("There was an error when decoding password from base64: {}", hashed.err().unwrap());
        return Ok(());
    }
    let hashed = hashed.unwrap();
    let hashed = hashed.as_slice();
    // start the bot
    let api = Api::new(settings.token);
    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // hash the text
                let mut hasher = Sha256::new();
                sha2::digest::Update::update(&mut hasher, data.as_bytes());
                let result = hasher.finalize();
                // compare them
                if compare_arrays(result.as_slice(), hashed) {
                    // send back only if they match
                    let mut to_send: String;
                    match get_ip() {
                        Ok(ip) => to_send = ip,
                        Err(err) => to_send = err
                    }
                    api.send(message.text_reply(to_send)).await?;
                }
            }
        }
    };
    return Ok(());
}

/// Connects to api.ipify.org and returns your IP address
/// Note that this method does not use any proxies
fn get_ip() -> Result<String, String> {
    // at first, try to to connect to ipify
    match TcpStream::connect("api.ipify.org:80") {
        Ok(mut stream) => {
            // create the request header; This is a const value
            let msg = b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n";
            match stream.write(msg) { // write the header to stream
                Ok(_) => {
                    let stream = BufReader::new(stream); // wrap stream to buf reader to read it line by line
                    let mut lines = stream.lines().map(|l| l.unwrap());
                    // read the first line of stream to make sure we got 200
                    match lines.next() {
                        Some(line) => {
                            if !line.ends_with("200 OK") {
                                return Err(String::from(format!("Server didn't return code 200! Code {} is returned.", line)));
                            }
                        }
                        None => {
                            return Err(String::from("Empty body from server"));
                        }
                    }
                    // read all next lines until we reach an "empty line". After that, the next line is the IP address
                    loop {
                        match lines.next() {
                            Some(line) => {
                                if line.len() == 0 { // check if the have reached the empty line
                                    return Ok(lines.next().unwrap());
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
                Err(e) => { return Err(String::from(format!("Cannot write server's buffer: {}", e))); }
            }
        }
        Err(e) => {
            return Err(String::from(format!("Cannot connect to server: {}", e)));
        }
    };
    return Err(String::from("Empty body from server"));
}

/// Compares two arrays from https://users.rust-lang.org/t/leakless-comparison-of-byte-arrays-c-vs-rust/15462
fn compare_arrays(a: &[u8], b: &[u8]) -> bool {
    let diff = a.iter()
        .zip(b.iter())
        .map(|(a, b)| a ^ b)
        .fold(0, |acc, x| acc | x);
    diff == 0 && a.len() == b.len()
}