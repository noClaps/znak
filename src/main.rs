use std::{env::args, fs, process::exit};

use crate::{
    errors::{Error, Result},
    parser::{parse_block, parse_frontmatter},
    render::render,
};

mod errors;
mod parser;
mod render;
#[cfg(test)]
mod tests;
mod token;

const USAGE: &str = "\
Usage: znak <source> [--frontmatter]

Arguments:
  <source>           The source file to compile to HTML

Options:
  -f, --frontmatter  Get the frontmatter, if any, as JSON
  -h, --help         Show this help and exit
";

pub fn main() {
    let mut show_fm = false;

    let mut args: Vec<_> = args().skip(1).collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) || args.is_empty() {
        println!("{USAGE}");
        exit(0);
    }
    if let Some(p) = args.iter().position(|f| f == "-f" || f == "--frontmatter") {
        show_fm = true;
        args.remove(p);
    }
    if let flags = args
        .iter()
        .filter(|f| f.starts_with('-'))
        .cloned()
        .collect::<Vec<_>>()
        && !flags.is_empty()
    {
        println!(
            "\
Unknown flags: {}
Usage: znak <source> [--frontmatter]
See znak --help for more details
",
            flags.join(" ")
        );
        exit(1);
    }
    if args.len() > 1 {
        println!(
            "\
Extra arguments provided: {}
Usage: znak <source> [--frontmatter]
See znak --help for more details
",
            args[1..].join(" ")
        );
        exit(1);
    }

    let source = {
        let source_file = args[0].clone();
        match fs::read_to_string(&source_file) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("Error reading from {source_file}: {err}");
                exit(1);
            }
        }
    };
    if show_fm {
        let fm = match frontmatter(&source) {
            Ok(fm) => fm,
            Err(err) => {
                eprintln!("{err}");
                exit(1);
            }
        };
        print!("{{");
        for (i, (k, v)) in fm.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            print!(r#""{k}":"{v}""#);
        }
        println!("}}");
        exit(0);
    }
    match compile(&source) {
        Ok(html) => println!("{html}"),
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
}

pub(crate) fn frontmatter(source: &str) -> Result<Vec<(String, String)>> {
    let source: Vec<_> = source.lines().collect();
    match source[0] {
        "---" => {
            let mut frontmatter = Vec::new();
            let mut i = 1;
            while source[i] != "---" {
                frontmatter.push(source[i]);
                i += 1;
            }
            parse_frontmatter(&frontmatter.join("\n"))
        }
        _ => Err(Error::other("Source has no frontmatter")),
    }
}

pub(crate) fn compile(source: &str) -> Result<String> {
    let mut source: Vec<_> = source.lines().collect();
    if source[0] == "---" {
        let mut i = 1;
        while source[i] != "---" {
            i += 1;
        }
        i += 1;
        source = source[i..].to_vec();
    }

    let parsed = parse_block(&source.join("\n"))?;
    Ok(render(parsed))
}
