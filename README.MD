# IP Sender Rust
Send your IP address via a Telegram bot.
## Advantages
* Easy on RAM: Rust makes this bot occupy as much as less ram possible.
* Ignores system proxy: It doesn't matter if your system has a proxy configured, this bot will get your real IP address.
* Fast
## Disadvantages
* No Proxy Support (for bot)
* Weak password hashing algorithm (SHA-256)
## Usage
Clone this repository and run `cargo build --release` to build the binaries for your system.

Then download [this](https://github.com/HirbodBehnam/IP-Sender-Rust/blob/master/config.json) file in the executable directory; Fill your bot token. For password, you can use [this](https://approsto.com/sha-generator/) site and copy the base64 format.

At last run your bot as a service.