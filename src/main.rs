use hex;
use sha2::{Sha256, Digest};
use tokio::sync::RwLock;
use lazy_static::lazy_static;
use teloxide::prelude::*;

lazy_static! {
    static ref HASHED_PASSWORD: RwLock<Vec<u8>> = RwLock::new(Vec::new());
    static ref REQWEST_CLINET: reqwest::Client = reqwest::Client::builder().no_proxy().build().unwrap();
}

#[tokio::main]
async fn main() {
    read_config().await;
    run().await;
}

async fn read_config() {
    let password = std::env::var("PASSWORD").expect("please specify password with \"PASSWORD\" environment variable");
    // Decode password to u8 vector
    let hashed = hex::decode(password).expect("cannot decode hex");
    *HASHED_PASSWORD.write().await = hashed;
}

async fn run() {
    let bot = Bot::from_env().auto_send();
    teloxide::repl(bot, |message| async move {
        if message.update.text() == None {
            return respond(());
        }
        // Hash the text
        let mut hasher = Sha256::new();
        sha2::digest::Update::update(&mut hasher, message.update.text().unwrap().as_bytes());
        let result = hasher.finalize();
        // Send the ip
        if compare_arrays(result.as_slice(), &HASHED_PASSWORD.read().await) {
            let result = match get_ip().await {
                Ok(ip) => ip,
                Err(err) => err.to_string(),
            };
            message.answer(result).await?;
        }
        respond(())
    })
    .await;
}

async fn get_ip() -> Result<String, reqwest::Error> {
    REQWEST_CLINET.get("https://api.ipify.org/")
        .send()
        .await?
        .text()
        .await
}

/// Compares two arrays from https://users.rust-lang.org/t/leakless-comparison-of-byte-arrays-c-vs-rust/15462
fn compare_arrays(a: &[u8], b: &[u8]) -> bool {
    let diff = a.iter()
        .zip(b.iter())
        .map(|(a, b)| a ^ b)
        .fold(0, |acc, x| acc | x);
    diff == 0 && a.len() == b.len()
}