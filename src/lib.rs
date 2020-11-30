#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::Path;

lazy_static! {
    static ref UA: regex::Regex = {
        let regex = match Path::new("crawler-user-agents.json").exists() {
            true => {
                let json =
                    fs::read_to_string("crawler-user-agents.json").expect("Unable to read file");
                let data: Vec<Value> = serde_json::from_str(&json).unwrap();

                let mut ret: String = String::from("(");

                ret.push_str(
                    data.iter()
                        .map(|i| i.get("pattern").unwrap().to_string().replace("\"", ""))
                        .collect::<Vec<String>>()
                        .join("|")
                        .as_str(),
                );

                ret.push(')');

                Some(ret)
            }
            false => {
                println!("crawler-user-agents.json does not exist! Get a copy from: https://github.com/monperrus/crawler-user-agents");
                None
            }
        };

        Regex::new(&regex.unwrap()).unwrap()
    };
}

pub fn bot(user_agent: &str) -> bool {
    UA.is_match(user_agent)
}
