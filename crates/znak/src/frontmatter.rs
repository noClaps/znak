use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseError {
    cause: String,
}
impl ParseError {
    fn new(cause: impl Into<String>) -> Self {
        Self {
            cause: cause.into(),
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing frontmatter: {}", self.cause)
    }
}
impl Error for ParseError {}

/// A function that returns the frontmatter in the given input text.
///
/// # Arguments
///
/// `input`: The input text to extract the frontmatter from. This can be from
/// a Markdown file as long as the syntax is supported by Znak. See the
/// [documentation](https://github.com/noClaps/znak/blob/main/docs/syntax.md)
/// for the supported syntax.
///
/// # Returns
///
/// Returns a map of frontmatter keys and values.
pub fn parse_frontmatter(input: impl Into<String>) -> Result<HashMap<String, String>, ParseError> {
    let input = input.into();
    let mut fm_vals = HashMap::new();
    let mut lines = input.trim().lines();

    match lines.next() {
        None => return Err(ParseError::new("Input is empty")),
        Some(first) if first != "---" => {
            return Err(ParseError::new(format!("No frontmatter found: {}", input)));
        }
        Some(_) => (),
    };

    for line in lines {
        if line == "---" {
            break;
        }
        let (key, val) = match line.split_once(':') {
            Some(split) => split,
            None => {
                return Err(ParseError::new(format!(
                    "Line is formatted incorrectly, missing `:` between key and value: {}",
                    input
                )));
            }
        };
        fm_vals.insert(key.trim().to_string(), val.trim().to_string());
    }

    Ok(fm_vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(vals: Vec<(impl Into<String>, impl Into<String>)>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (key, val) in vals {
            map.insert(key.into(), val.into());
        }
        map
    }

    #[test]
    fn frontmatter() {
        let fm = parse_frontmatter(
            r#"
---
title: A title
description: Some description here
date: 2025-03-11
---
"#
            .to_string(),
        )
        .unwrap();
        let check = make_map(vec![
            ("title", "A title"),
            ("description", "Some description here"),
            ("date", "2025-03-11"),
        ]);
        assert_eq!(fm, check);

        let fm = parse_frontmatter(
            r#"
---
title: A title
description: Some description here
date: 2025-03-11
---

Some extra text here.
"#
            .to_string(),
        )
        .unwrap();
        let check = make_map(vec![
            ("title", "A title"),
            ("description", "Some description here"),
            ("date", "2025-03-11"),
        ]);
        assert_eq!(fm, check);

        let fm = parse_frontmatter(
            r#"
---
title: Google: A Misrepresented Evil
---

This was a post about Google. There's also a <hr /> below to see what happens

---
"#
            .to_string(),
        )
        .unwrap();
        let check = make_map(vec![("title", "Google: A Misrepresented Evil")]);
        assert_eq!(fm, check);

        let fm = parse_frontmatter(r#"
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
"#.to_string()).unwrap();
        let check = make_map(vec![
            ("title", "Intro to Privacy, Security and Anonymity"),
            (
                "description",
                "How to protect yourself from the internet, on the internet",
            ),
            ("date", "2022-04-06"),
            ("lastmod", "2023-03-09"),
        ]);
        assert_eq!(fm, check);
    }
}
