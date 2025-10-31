use std::collections::HashMap;

use regex::Regex;

/// A heading. This was adapted from [Astro](https://astro.build)'s
/// MarkdownHeading type.
#[derive(Debug, PartialEq)]
pub struct Heading {
    /// The depth of the heading element. A h1 would have a depth of 1, for
    /// example.
    pub depth: usize,
    /// The slug, or ID, of the heading element.
    pub slug: String,
    /// The content of the heading element.
    pub title: String,
}

/// A function that returns the [Heading]s in the given input text.
///
/// # Parameters
///
/// - `input`: The input text to extract the headings from. This can be from a
///   Markdown file as long as the syntax is supported by Znak. See the
///   [documentation](index.html#syntax) for the supported syntax.
pub fn parse_headings(input: impl Into<String>) -> Vec<Heading> {
    let input = input.into();
    let mut slugger = Slugger::new();
    let re = Regex::new("(?m)^(#{1,6}) (.+)").unwrap(); // can unwrap as known safe regex

    for re_match in re.captures_iter(&input) {
        let level = match re_match.get(1) {
            Some(level) => level,
            None => continue,
        }
        .len()
        .try_into()
        .unwrap();
        let heading = match re_match.get(2) {
            Some(heading) => heading,
            None => continue,
        }
        .as_str()
        .to_string();
        slugger.slug(heading, level);
    }

    return slugger.headings;
}

pub(crate) struct Slugger {
    occurrences: HashMap<String, u64>,
    pub(crate) headings: Vec<Heading>,
}

impl Slugger {
    pub(crate) fn new() -> Self {
        Self {
            occurrences: HashMap::new(),
            headings: vec![],
        }
    }

    pub(crate) fn slug(&mut self, heading: String, depth: usize) -> String {
        let heading_re = Regex::new("[^a-zA-Z0-9]").unwrap(); // can unwrap as known safe regex
        let clean_heading = heading_re.replace_all(&heading, "-").to_lowercase();
        let mut slug = clean_heading.clone();

        match self.occurrences.get(&clean_heading) {
            Some(occ) => {
                slug += format!("-{occ}").as_str();
                self.occurrences.insert(clean_heading, occ + 1);
            }
            None => {
                self.occurrences.insert(clean_heading, 1);
            }
        };

        self.headings.push(Heading {
            depth,
            slug: slug.clone(),
            title: heading,
        });
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings() {
        let headings = parse_headings("## Heading 2".to_string());
        assert_eq!(
            headings,
            vec![Heading {
                depth: 2,
                slug: "heading-2".to_string(),
                title: "Heading 2".to_string()
            }]
        );

        let headings = parse_headings("### This_is-a🍪heading".to_string());
        assert_eq!(
            headings,
            vec![Heading {
                depth: 3,
                slug: "this-is-a-heading".to_string(),
                title: "This_is-a🍪heading".to_string(),
            }]
        );
    }
}
