use crate::errors::{Error, Result};

mod block;
mod inline;
mod slugger;

pub use block::parse_block;

pub fn parse_frontmatter(source: &str) -> Result<Vec<(String, String)>> {
    let mut frontmatter = vec![];
    for line in source.lines() {
        match line.split_once('=') {
            Some((k, v)) => frontmatter.push((k.trim().to_string(), v.trim().to_string())),
            None => {
                return Err(Error::frontmatter(
                    line,
                    "Line is missing an equal sign `=`",
                ));
            }
        }
    }
    Ok(frontmatter)
}
