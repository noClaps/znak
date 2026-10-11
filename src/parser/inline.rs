use crate::{
    errors::{Error, Result},
    token::Token,
};

#[allow(clippy::too_many_lines)]
pub fn parse_inline(line: &str) -> Result<Vec<Token>> {
    let chars: Vec<_> = line.chars().collect();
    let mut i = 0;
    let mut tokens = vec![];
    let mut plaintext = String::new();

    macro_rules! push_text {
        () => {
            if !plaintext.is_empty() {
                tokens.push(Token::text(&plaintext));
                plaintext.clear();
            }
        };
    }

    macro_rules! parse_inline {
        ($delim:literal, $el:literal) => {
            if chars[i] == $delim {
                push_text!();
                i += 1;
                let mut contents = String::new();
                while i < chars.len() && chars[i] != $delim {
                    contents.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::element($el, &[], &parse_inline(&contents)?));
                i += 1;
                continue;
            }
        };
    }

    while i < chars.len() {
        // line break
        if chars[i..].starts_with(&['\\', ' ']) {
            push_text!();
            tokens.push(Token::element("br", &[], &[]));
            i += 2;
            continue;
        }
        // escape
        if chars[i] == '\\' {
            push_text!();
            i += 1;
            if i < chars.len() {
                tokens.push(Token::text(&chars[i].to_string()));
            }
            i += 1;
            continue;
        }
        // bold
        parse_inline!('*', "strong");
        // italics
        parse_inline!('/', "em");
        // strikethrough
        parse_inline!('~', "s");
        // subscript
        parse_inline!('_', "sub");
        // superscript
        parse_inline!('^', "sup");
        // highlight
        parse_inline!('=', "mark");
        // highlighted code
        if chars[i..].starts_with(&['`', '`']) {
            push_text!();
            i += 2;
            let mut contents = String::new();
            while i + 1 < chars.len() && chars[i..=i + 1] != ['`', '`'] {
                contents.push(chars[i]);
                i += 1;
            }
            let Some((lang, code)) = contents.split_once(' ') else {
                return Err(Error::content(
                    line,
                    "Expected language for syntax highlighting. Use a single backtick if you don't want to specify a language.",
                ));
            };
            tokens.push(Token::element(
                "code",
                &[("data-lang", lang)],
                &[Token::text(code)],
            ));
            i += 2;
            continue;
        }
        // code
        if chars[i] == '`' {
            push_text!();
            i += 1;
            let mut contents = String::new();
            while i < chars.len() && chars[i] != '`' {
                contents.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::element("code", &[], &[Token::text(&contents)]));
            i += 1;
            continue;
        }
        // math
        if chars[i] == '$' {
            push_text!();
            i += 1;
            let mut contents = String::new();
            while i < chars.len() && chars[i] != '$' {
                contents.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::element(
                "span",
                &[("class", "math inline")],
                &[Token::text(&contents)],
            ));
            i += 1;
            continue;
        }
        // links
        if chars[i] == '['
            && let Some(p) = chars[i..].windows(2).position(|w| w == [']', '<'])
            && chars[i + p..].contains(&'>')
        {
            push_text!();
            i += 1;
            let mut text = String::new();
            let mut depth = 0;
            while i < chars.len() {
                match (chars[i], depth) {
                    (']', 0) => break,
                    (']', 1..) => depth -= 1,
                    ('[', _) => depth += 1,
                    _ => (),
                }
                text.push(chars[i]);
                i += 1;
            }
            i += 2;
            let mut href = String::new();
            while i < chars.len() && chars[i] != '>' {
                href.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::element(
                "a",
                &[("href", &href)],
                &parse_inline(&text)?,
            ));
            i += 1;
            continue;
        }
        // bare link
        if chars[i] == '<' && chars[i + 1..].contains(&'>') {
            push_text!();
            i += 1;
            let mut href = String::new();
            while i < chars.len() && chars[i] != '>' {
                href.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::element(
                "a",
                &[("href", &href)],
                &[Token::text(&href)],
            ));
            i += 1;
            continue;
        }

        plaintext.push(chars[i]);
        i += 1;
    }

    push_text!();
    Ok(tokens)
}
