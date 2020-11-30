# Voight Kampff ![Crates.io](https://img.shields.io/crates/v/voight_kampff)


This crate is a rust version of this [excellent ruby gem](https://github.com/biola/Voight-Kampff) for figuring out if a http request is from a bot/crawler/scraper/replicant. 

It relies only on the user agent that was sent and does no other checks on its own. 

### Usage
`voight_kampff::bot()` will return true if the user agent matches a user agent in the `crawler-user-agents.json` file


In your Cargo.toml: 
```toml
voight_kampff = "0.1.1"
```

In your code: 
```rust
use voight_kampff;

voight_kampff::bot("Mozilla/5.0 ...");
```

As with the ruby version, this crate gets its list of user agents from [this repo](https://github.com/monperrus/crawler-user-agents) by monperrus.



