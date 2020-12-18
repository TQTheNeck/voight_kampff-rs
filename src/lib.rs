#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::Path;

lazy_static! {
    static ref UA: Result<regex::Regex, regex::Error> = {
        let regex = match Path::new("crawler-user-agents.json").exists() {
            true => {
                let json =
                    fs::read_to_string("crawler-user-agents.json").expect("Unable to read crawler-user-agents.json");
                let data: Vec<Value> = serde_json::from_str(&json).expect("Error converting json with serde_json");

                let mut ret: String = String::from("(");

                ret.push_str(
                    data.iter()
                        .map(|item| item.get("pattern").unwrap().to_string().replace("\"", ""))
                        .collect::<Vec<String>>()
                        .join("|")
                        .as_str(),
                );

                ret.push(')');

                Some(ret)
            }
            false => None,
        };

        match regex {
            Some(r) => Regex::new(&r),
            None => {
                panic!("crawler-user-agents.json does not exist in this directory! Get a copy from: https://github.com/monperrus/crawler-user-agents");
            }
        }

        //Regex::new(&regex.unwrap())
    };
}

/// Checks if a user agent is in the list of bots, if so it returns true.
///
/// # Example
/// ```
/// use voight_kampff;
///
/// assert_eq!(voight_kampff::bot("DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html"), true);
/// ```
pub fn bot(user_agent: &str) -> bool {
    match UA.as_ref() {
        Ok(r) => r.is_match(user_agent),
        Err(e) => {
            panic!("Regex error: {}", e);
        }
    }
}
