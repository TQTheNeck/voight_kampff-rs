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

#[cfg(test)]
mod tests {

    #[test]
    fn bot() {
        assert_eq!(crate::bot("DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"), true);
    }

    #[test]
    fn not_bot() {
        assert_eq!(crate::bot("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.66 Safari/537.36"), false);
    }
}
