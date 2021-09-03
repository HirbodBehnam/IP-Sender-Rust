# IP Sender Rust
Send your IP address via a Telegram bot.
## Advantages
* Easy on RAM: Rust makes this bot occupy as much as less ram possible.
* Fast
## Disadvantages
* Weak password hashing algorithm (SHA-256)
## Usage
Clone this repository and run `cargo build --release` to build the binaries for your system.

Before running the bot, you must set these environment variables to run the bot. The password is the hexed sha256 password.
```
export PASSWORD=a619432efe5b515c002c25c6d1c24976245f1993e6d991e6929d598114624095
export TELOXIDE_TOKEN=12345abcd
export TELOXIDE_PROXY=http://127.0.0.1:10809
```

At last run your bot as a service.