use std::collections::HashMap;

/// A function that returns the frontmatter in the given input text as a map.
///
/// # Parameters
///
/// - `input`: The input text to extract the frontmatter from. This can be from
///   a Markdown file as long as the syntax is supported by Znak. See the
///   [documentation](index.html#syntax) for the supported syntax.
///
/// # Usage
///
/// ```rust
/// use znak::parse_frontmatter;
///
/// let input = include_str!("../demo.md");
/// let frontmatter = parse_frontmatter(input).unwrap();
/// ```
pub fn parse_frontmatter(input: &str) -> Result<HashMap<String, String>, String> {
    let mut fm_vals = HashMap::new();
    let mut lines = input.trim().lines();

    match lines.next() {
        None => return Err(format!("Input is empty")),
        Some(first) if first != "---" => {
            return Err(format!("No frontmatter found: {}", input));
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
                return Err(format!(
                    "Line is formatted incorrectly, missing `:` between key and value: {}",
                    input
                ));
            }
        };
        fm_vals.insert(key.trim().to_string(), val.trim().to_string());
    }

    Ok(fm_vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_map {
        ($($key:literal: $value:literal$(,)?)*) => {
            std::collections::HashMap::from([
                $(($key.to_string(), $value.to_string()),)*
            ])
        };
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
"#,
        )
        .unwrap();
        let check = make_map! {
            "title": "A title",
            "description": "Some description here",
            "date": "2025-03-11"
        };
        assert_eq!(fm, check);

        let fm = parse_frontmatter(
            r#"
---
title: A title
description: Some description here
date: 2025-03-11
---

Some extra text here.
"#,
        )
        .unwrap();
        let check = make_map! {
            "title": "A title",
            "description": "Some description here",
            "date": "2025-03-11",
        };
        assert_eq!(fm, check);

        let fm = parse_frontmatter(
            r#"
---
title: Google: A Misrepresented Evil
---

This was a post about Google. There's also a <hr /> below to see what happens

---
"#,
        )
        .unwrap();
        let check = make_map! {"title": "Google: A Misrepresented Evil"};
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
"#).unwrap();
        let check = make_map! {
            "title": "Intro to Privacy, Security and Anonymity",
            "description": "How to protect yourself from the internet, on the internet",
            "date": "2022-04-06",
            "lastmod": "2023-03-09",
        };
        assert_eq!(fm, check);
    }
}
