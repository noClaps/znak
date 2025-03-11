use std::collections::HashMap;

pub fn parse_frontmatter(input: String) -> Option<HashMap<String, String>> {
    let mut fm_vals = HashMap::new();
    let lines = input.trim().lines().collect::<Vec<&str>>();
    if lines[0] != "---" {
        eprintln!("No frontmatter found");
        return None;
    };

    let mut cur = 1;
    while cur < lines.len() && lines[cur] != "---" {
        let line = lines[cur];
        let (key, value) = match line.split_once(':') {
            Some((key, val)) => (key.trim(), val.trim()),
            None => continue,
        };
        fm_vals.insert(key.to_string(), value.to_string());

        cur += 1;
    }

    Some(fm_vals)
}
