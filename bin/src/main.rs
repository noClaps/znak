use std::{io::Read, process::exit};

use clap::Parser;
use clio::Input;
use znak_lang::{headings, render};

#[derive(Debug, Parser)]
#[command(version, name = "znak")]
struct Args {
    /// Path to the Znak file to build to HTML
    #[arg(value_parser)]
    input: Input,

    /// Path to theme TOML file, leave empty if you don't want syntax highlighting
    #[arg(value_parser, short, long)]
    theme: Option<Input>,

    /// Whether or not the CLI should return headings
    #[arg(long)]
    headings: bool,
}

fn main() {
    let mut args = Args::parse();

    let mut input = String::new();
    match args.input.read_to_string(&mut input) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERROR: Error parsing input file: {err}");
            exit(1)
        }
    }

    if args.headings {
        match serde_json::to_string(&headings(input)) {
            Ok(headings) => println!("{headings}"),
            Err(err) => {
                eprintln!("ERROR: Error serialising headings: {err}");
                exit(1)
            }
        };
        return;
    }

    let mut theme_file = String::new();
    match args.theme {
        None => {
            theme_file = include_str!("../../theme.toml").to_string();
        }
        Some(mut theme) => match theme.read_to_string(&mut theme_file) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("ERROR: Error parsing theme file: {err}");
                exit(1)
            }
        },
    }

    let theme = match znak_lang::Theme::new(theme_file) {
        Ok(theme) => theme,
        Err(err) => {
            eprintln!("Error generating theme: {err}");
            exit(1)
        }
    };

    println!("{}", render(input, theme))
}
