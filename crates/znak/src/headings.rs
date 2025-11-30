use std::collections::HashMap;

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
///
/// # Usage
///
/// ```rust
/// use znak::parse_headings;
///
/// let input = include_str!("../demo.md");
/// let headings = parse_headings(input);
/// ```
pub fn parse_headings(input: &str) -> Vec<Heading> {
    let mut slugger = Slugger::new();
    for line in input.lines() {
        let mut line = line.chars();
        let mut level = 0;
        while let Some('#') = line.next() {
            level += 1;
        }
        let heading = line.collect::<String>();
        slugger.slug(heading.trim(), level);
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

    pub(crate) fn slug(&mut self, heading: &str, depth: usize) -> String {
        let clean_heading = heading
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .to_lowercase();
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
            title: heading.to_string(),
        });
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings() {
        let headings = parse_headings("## Heading 2");
        assert_eq!(
            headings,
            vec![Heading {
                depth: 2,
                slug: "heading-2".to_string(),
                title: "Heading 2".to_string()
            }]
        );

        let headings = parse_headings("### This_is-a🍪heading");
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
