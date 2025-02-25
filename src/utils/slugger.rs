use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

/// A heading. This was adapted from Astro's MarkdownHeading type
#[derive(Debug, Serialize)]
pub struct Heading {
    /// The depth of the heading element. A h1 would have a depth of 1, for example.
    pub depth: u8,
    /// The slug, or ID, of the heading element.
    pub slug: String,
    /// The content of the heading element.
    pub title: String,
}

pub struct Slugger {
    occurrences: HashMap<String, u64>,
    pub headings: Vec<Heading>,
}

impl Slugger {
    pub fn new() -> Slugger {
        Slugger {
            occurrences: HashMap::new(),
            headings: vec![],
        }
    }

    pub fn slug(&mut self, heading: String, depth: u8) -> String {
        let heading_re = match Regex::new(r"[^a-zA-Z0-9]") {
            Ok(re) => re,
            Err(_) => unreachable!(),
        };
        let clean_heading = heading_re.replace_all(&heading, "-").to_lowercase();
        let mut slug = clean_heading.clone();

        match self.occurrences.get(&clean_heading) {
            Some(occ) => {
                slug += &format!("-{occ}",);
                self.occurrences.insert(clean_heading.clone(), occ + 1);
            }
            None => {
                self.occurrences.insert(clean_heading.clone(), 1);
            }
        };

        self.headings.push(Heading {
            depth,
            slug: slug.clone(),
            title: heading,
        });

        return slug;
    }
}
