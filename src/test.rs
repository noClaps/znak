use highlight::highlight;
use pulldown_latex::{config::DisplayMode, push_mathml, Parser, RenderConfig, Storage};
use std::collections::HashMap;

use crate::{frontmatter, headings, render, Heading, Theme};

fn test_render(input: &str, test: &str) {
    let github_dark_toml = include_str!("../theme.toml");
    let github_dark = match Theme::new(github_dark_toml.to_string()) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let output = render(input.to_string(), github_dark);
    assert_eq!(output, test.to_string())
}

fn test_render_ne(input: &str, test: &str) {
    let github_dark_toml = include_str!("../theme.toml");
    let github_dark = match Theme::new(github_dark_toml.to_string()) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let output = render(input.to_string(), github_dark);
    assert_ne!(output, test.to_string())
}

fn test_render_one_of(input: &str, tests: Vec<&str>) {
    let github_dark_toml = include_str!("../theme.toml");
    let github_dark = match Theme::new(github_dark_toml.to_string()) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let output = render(input.to_string(), github_dark);
    for test in tests {
        if output == test {
            return;
        }
    }

    panic!("Test failed!")
}

fn test_headings(input: &str, test: Vec<Heading>) {
    let headings = headings(input.to_string());
    assert_eq!(headings.len(), test.len());
    for i in 0..headings.len() {
        assert_eq!(headings[i].depth, test[i].depth);
        assert_eq!(headings[i].slug, test[i].slug);
        assert_eq!(headings[i].title, test[i].title);
    }
}

fn test_frontmatter(input: &str, test: HashMap<String, String>) {
    let fm = frontmatter(input.to_string());
    assert_eq!(fm, Some(test));
}

fn render_math(input: &str, display_mode: DisplayMode) -> String {
    let storage = Storage::new();
    let parser = Parser::new(input, &storage);
    let mut output = String::new();
    let config = RenderConfig {
        display_mode,
        ..Default::default()
    };

    match push_mathml(&mut output, parser, config) {
        Ok(_) => output,
        Err(err) => panic!("{err}"),
    }
}

#[test]
fn headings_test() {
    test_render("# Heading 1", "<h1 id=\"heading-1\">Heading 1</h1>");
    test_render("## Heading 2", "<h2 id=\"heading-2\">Heading 2</h2>");
    test_render("### Heading 3", "<h3 id=\"heading-3\">Heading 3</h3>");
    test_render("#### Heading 4", "<h4 id=\"heading-4\">Heading 4</h4>");
    test_render("##### Heading 5", "<h5 id=\"heading-5\">Heading 5</h5>");
    test_render("###### Heading 6", "<h6 id=\"heading-6\">Heading 6</h6>");
    test_headings(
        "## Heading 2",
        vec![Heading {
            depth: 2,
            slug: "heading-2".to_string(),
            title: "Heading 2".to_string(),
        }],
    );
    test_headings(
        "### This_is-a🍪heading",
        vec![Heading {
            depth: 3,
            slug: "this-is-a-heading".to_string(),
            title: "This_is-a🍪heading".to_string(),
        }],
    );
    test_render(
        "
## Heading
### Heading
#### Heading
",
        r#"<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>"#,
    );
}

#[test]
fn horizontal_rule() {
    test_render("---", "<hr />");
    test_render("-----", "<hr />");
    test_render_ne("-------asdsa", "<hr />");
}

#[test]
fn inline_formatting() {
    test_render("**bold text**", "<p><strong>bold text</strong></p>");
    test_render("_italic text_", "<p><em>italic text</em></p>");
    test_render(
        "_**bold and italic text**_",
        "<p><em><strong>bold and italic text</strong></em></p>",
    );
    test_render(
        "**_bold and italic text_**",
        "<p><strong><em>bold and italic text</em></strong></p>",
    );
    test_render(
        "This is some `inline code`",
        "<p>This is some <code>inline code</code></p>",
    );
    test_render(
        "This is a [link](https://zerolimits.dev)",
        r#"<p>This is a <a href="https://zerolimits.dev">link</a></p>"#,
    );
    test_render(
        "This is a **[bold link](https://zerolimits.dev)**",
        r#"<p>This is a <strong><a href="https://zerolimits.dev">bold link</a></strong></p>"#,
    );
    test_render(
        "This is a [**bold link**](https://zerolimits.dev)",
        r#"<p>This is a <a href="https://zerolimits.dev"><strong>bold link</strong></a></p>"#,
    );
    test_render(
        "This is an _[italic link](https://zerolimits.dev)_",
        r#"<p>This is an <em><a href="https://zerolimits.dev">italic link</a></em></p>"#,
    );
    test_render(
        "This is an [_italic link_](https://zerolimits.dev)",
        r#"<p>This is an <a href="https://zerolimits.dev"><em>italic link</em></a></p>"#,
    );
    test_render(
        "This is a `[code link](https://zerolimits.dev)`",
        r#"<p>This is a <code>[code link](https://zerolimits.dev)</code></p>"#,
    );
    test_render(
        "This is a [`code link`](https://zerolimits.dev)",
        r#"<p>This is a <a href="https://zerolimits.dev"><code>code link</code></a></p>"#,
    );
    test_render(
        "This is formatting inside a `**code** _block_`",
        "<p>This is formatting inside a <code>**code** _block_</code></p>",
    );
    test_render("~~strikethrough~~", "<p><s>strikethrough</s></p>");
    test_render(
        "This is a sentence with ~~strikethrough~~ in it",
        "<p>This is a sentence with <s>strikethrough</s> in it</p>",
    );
    test_render("==highlight==", "<p><mark>highlight</mark></p>");
    test_render(
        "This is a sentence with ==highlight== in it",
        "<p>This is a sentence with <mark>highlight</mark> in it</p>",
    );
    test_render("~sub~script", "<p><sub>sub</sub>script</p>");
    test_render(
        "This is a sentence with ~sub~script in it",
        "<p>This is a sentence with <sub>sub</sub>script in it</p>",
    );
    test_render("^super^script", "<p><sup>super</sup>script</p>");
    test_render(
        "This is a sentence with ^super^script in it",
        "<p>This is a sentence with <sup>super</sup>script in it</p>",
    );
    test_render(
        "[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)",
        r#"<p><a href="https://en.wikipedia.org/wiki/Rust_(programming_language)">link with parentheses</a></p>"#,
    );
    test_render(
        "[Some [square braces] inside a link](https://zerolimits.dev)",
        r#"<p><a href="https://zerolimits.dev">Some [square braces] inside a link</a></p>"#,
    );

    test_render(
        "Inline $$x+y$$ math",
        r#"<p>Inline <math display="inline"><mi>x</mi><mo>+</mo><mi>y</mi></math> math</p>"#,
    );
    test_render(
        "This is a **line** with multiple **bold** words",
        "<p>This is a <strong>line</strong> with multiple <strong>bold</strong> words</p>",
    );
    test_render(
        "This is some __underlined__ text",
        "<p>This is some <u>underlined</u> text</p>",
    );
    test_render(
        "This is some ___italic text_ in some underlined__ text",
        "<p>This is some <u><em>italic text</em> in some underlined</u> text</p>",
    );
    test_render(
        "Special characters: “_voilà_!”",
        "<p>Special characters: “<em>voilà</em>!”</p>",
    );
}

#[test]
fn blockquotes() {
    test_render(
        "> This is text in a blockquote",
        "<blockquote><p>This is text in a blockquote</p></blockquote>",
    );
    test_render(
        "
> This is a
>
> multiline blockquote
",
        "<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>",
    );
}

#[test]
fn images() {
    test_render_one_of(
        "![alt text](https://picsum.photos/300)",
        vec![
            r#"<figure><img src="https://picsum.photos/300" alt="alt text" /><figcaption>alt text</figcaption></figure>"#,
            r#"<figure><img alt="alt text" src="https://picsum.photos/300" /><figcaption>alt text</figcaption></figure>"#,
        ],
    );
    test_render_one_of(
        "![This contains a [link](https://picsum.photos)](https://picsum.photos/300)",
        vec![
            r#"<figure><img src="https://picsum.photos/300" alt="This contains a [link](https://picsum.photos)" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>"#,
            r#"<figure><img alt="This contains a [link](https://picsum.photos)" src="https://picsum.photos/300" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>"#,
        ],
    );
}

#[test]
fn code_blocks() {
    let github_dark_toml = include_str!("../theme.toml");
    let github_dark = match Theme::new(github_dark_toml.to_string()) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("{}", err)
        }
    };

    test_render(
        r#"
```py
print("Your code here")
```
"#,
        highlight(
            r#"print("Your code here")"#.to_string(),
            "py".to_string(),
            github_dark.clone(),
        )
        .as_str(),
    );
    test_render(
        "
```
This is some text in a code block
```
",
        highlight(
            "This is some text in a code block".to_string(),
            "plaintext".to_string(),
            github_dark.clone(),
        )
        .as_str(),
    );
    test_render(
        "
```skajdlas
This is for a language that doesn't exist
```
",
        highlight(
            "This is for a language that doesn't exist".to_string(),
            "plaintext".to_string(),
            github_dark,
        )
        .as_str(),
    );
}

#[test]
fn lists() {
    test_render(
        "
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space or 1 tab indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want
",
        "<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
    );
    test_render(
        "
1. list item 1
2. list item 2
3. list item 3
	1. nested list item 1 (3 space or 1 tab indentation allowed)
	2. nested list item 2
		1. You can nest as far as you want
",
        "<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
    );
    test_render(
        "
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space or 1 tab indentation allowed)
  - nested list item 2
    - You can nest as far as you want
",
        "<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
    );
    test_render(
        "
- list item 1 (only - allowed for list)
- list item 2
- list item 3
	- nested list item 1 (2 space or 1 tab indentation allowed)
	- nested list item 2
		- You can nest as far as you want
",
        "<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
    );
}

#[test]
fn tables() {
    test_render(
        "
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
",
        r#"<table><thead><tr><th align="left">title</th><th align="center">description</th><th align="right">heading 1</th><th align="">heading 2</th></tr></thead><tbody><tr><td align="left">left-aligned</td><td align="center">center-aligned</td><td align="right">right-aligned</td><td align="">default text alignment</td></tr></tbody></table>"#,
    );
}

#[test]
fn html_elements() {
    test_render(
        "
<div>
Content here
</div>
",
        "<div>
Content here
</div>",
    );
    test_render(
        "<u>This element isn't closed",
        "<u>This element isn't closed",
    );
}

#[test]
fn math_blocks() {
    test_render(
        "
$$
a^2 + b^2 = c^2
$$
",
        render_math("a^2 + b^2 = c^2", DisplayMode::Block).as_str(),
    );
    test_render("$$", "<p>$$</p>");
}

#[test]
fn escaped_chars() {
    test_render(
        "This is \\**escaped bold**",
        "<p>This is **escaped bold**</p>",
    );
}

#[test]
fn containers() {
    test_render(
        "
::: note A NOTE
This is some text in a note.
:::
",
        r#"<div class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>"#,
    );
    test_render_one_of(
        r#"
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
"#,
        vec![
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a href="https://zerolimits.dev" rel="noopener noreferrer" target="_blank">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a rel="noopener noreferrer" href="https://zerolimits.dev" target="_blank">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a rel="noopener noreferrer" target="_blank" href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a target="_blank" rel="noopener noreferrer" href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote"><p class="quote-heading"><b><a target="_blank" href="https://zerolimits.dev" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
        ],
    );
    test_render_one_of(
        r#"
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
"#,
        vec![
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a href="https://zerolimits.dev" rel="noopener noreferrer" target="_blank">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a rel="noopener noreferrer" href="https://zerolimits.dev" target="_blank">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a rel="noopener noreferrer" target="_blank" href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a target="_blank" href="https://zerolimits.dev" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
            r#"<div class="znak-container quote bold"><p class="quote-heading"><b><a target="_blank" rel="noopener noreferrer" href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>"#,
        ],
    );
    test_render(
        "
::: warning
This is some text in a warning.
:::
",
        r#"<div class="znak-container warning"><p class="warning-heading"><b>WARNING</b></p><p>This is some text in a warning.</p></div>"#,
    );
    test_render(
        "
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
",
        r#"<div class="znak-container block1"><p class="block1-heading"><b>This is the outer container</b></p><p>You can have some text here.</p><div class="znak-container block2"><p class="block2-heading"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>"#,
    );
    test_render_one_of(
        r#"
::: note A NOTE {id="my-note"}
This is some text in a note.
:::
"#,
        vec![
            r#"<div id="my-note" class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>"#,
            r#"<div class="znak-container note" id="my-note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>"#,
        ],
    );
    test_render(
        "
::: note A NOTE
This is some text in a note.
:::

::: warning
This is some text in a warning.
:::
",
        r#"<div class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div><div class="znak-container warning"><p class="warning-heading"><b>WARNING</b></p><p>This is some text in a warning.</p></div>"#,
    );
}

#[test]
fn misc() {
    test_render(
        "
This is a multi line
string
",
        "<p>This is a multi linestring</p>",
    );
    test_render(
        "> This is quite a **bold** statement!",
        "<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>",
    );
}

#[test]
fn empty_blocks() {
    test_render(":::", "<p>:::</p>");
    test_render("```", "<p>```</p>");
    test_render("****", "<p>****</p>");
    test_render("__", "<p>__</p>");
    test_render("$$$$", "<p>$$$$</p>");
    test_render(
        "[](https://zerolimits.dev)",
        "<p>[](https://zerolimits.dev)</p>",
    );
    test_render("[link]()", "<p>[link]()</p>");
    test_render("^^", "<p>^^</p>");
    test_render("~~", "<p>~~</p>");
    test_render("====", "<p>====</p>");
    test_render("~~~~", "<p>~~~~</p>");
}

#[test]
fn front_matter() {
    let mut hm = HashMap::new();
    hm.insert("title".to_string(), "A title".to_string());
    hm.insert(
        "description".to_string(),
        "Some description here".to_string(),
    );
    hm.insert("date".to_string(), "2025-03-11".to_string());
    test_frontmatter(
        "
---
title: A title
description: Some description here
date: 2025-03-11
---
",
        hm.clone(),
    );

    test_frontmatter(
        "
---
title: A title
description: Some description here
date: 2025-03-11
---

Some extra text here.
",
        hm,
    );

    let mut hm2 = HashMap::new();
    hm2.insert(
        "title".to_string(),
        "Google: A Misrepresented Evil".to_string(),
    );
    test_frontmatter(
        "
---
title: Google: A Misrepresented Evil
---

This was a post about Google. There's also a <hr /> below to see what happens

---
",
        hm2,
    );

    let mut hm3 = HashMap::new();
    hm3.insert(
        "title".to_string(),
        "Intro to Privacy, Security and Anonymity".to_string(),
    );
    hm3.insert(
        "description".to_string(),
        "How to protect yourself from the internet, on the internet".to_string(),
    );
    hm3.insert("date".to_string(), "2022-04-06".to_string());
    hm3.insert("lastmod".to_string(), "2023-03-09".to_string());
    test_frontmatter("
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
", hm3);
    test_render("
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
", "<hr /><p>I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.</p>");
}
