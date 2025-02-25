use crate::utils::slugger::{Heading, Slugger};
use regex::Regex;

pub fn parse_headings(input: String) -> Vec<Heading> {
    let mut slugger = Slugger::new();

    let re = match Regex::new(r"^(#{1,6}) (.+)") {
        Ok(re) => re,
        Err(_) => {
            unreachable!()
        }
    };
    for (_, [level_str, heading]) in re.captures_iter(&input).map(|c| c.extract()) {
        let level = level_str.len();
        slugger.slug(heading.to_string(), level as u8);
    }

    slugger.headings
}
