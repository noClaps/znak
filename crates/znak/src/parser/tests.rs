use highlight::{Highlight, HighlightConfiguration};
use html::assert_html_eq;

use crate::render;

pub(crate) fn test_render(input: impl Into<String>, test: impl Into<String>) {
    let theme = include_str!("../../../../theme.css").parse().unwrap();
    let mut hl = Highlight::new(theme);
    let python = HighlightConfiguration::new(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        tree_sitter_python::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap();
    hl.add_language(&["python", "py"], python);

    let output = render(input.into(), &hl);
    assert_html_eq!(output, test.into());
}

#[test]
fn headings() {
    test_render("# Heading 1", "<h1 id=\"heading-1\">Heading 1</h1>");
    test_render("## Heading 2", "<h2 id=\"heading-2\">Heading 2</h2>");
    test_render("### Heading 3", "<h3 id=\"heading-3\">Heading 3</h3>");
    test_render("#### Heading 4", "<h4 id=\"heading-4\">Heading 4</h4>");
    test_render("##### Heading 5", "<h5 id=\"heading-5\">Heading 5</h5>");
    test_render("###### Heading 6", "<h6 id=\"heading-6\">Heading 6</h6>");
    test_render(
        r#"
## Heading
### Heading
#### Heading
"#,
        r#"<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>"#,
    );
}

#[test]
fn blockquotes() {
    test_render(
        "> This is quite a *bold* statement!",
        "<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>",
    );
    test_render(
        "> This is text in a blockquote",
        "<blockquote><p>This is text in a blockquote</p></blockquote>",
    );
    test_render(
        r#"
> This is a
>
> multiline blockquote
"#,
        "<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>",
    );
}

#[test]
fn paragraphs() {
    test_render(
        r#"
This is a multi line
string
"#,
        "<p>This is a multi linestring</p>",
    );
}

#[test]
fn horizontal_rule() {
    test_render("---", "<hr />");
    test_render("-----", "<hr />");
    test_render("-------asdsa", "<p>-------asdsa</p>");
}

#[test]
fn images() {
    test_render(
        "![alt text](https://picsum.photos/300)",
        "<figure><img src=\"https://picsum.photos/300\" alt=\"alt text\" /><figcaption>alt text</figcaption></figure>",
    );
    test_render(
        "![This contains a [link](https://picsum.photos)](https://picsum.photos/300)",
        "<figure><img src=\"https://picsum.photos/300\" alt=\"This contains a [link](https://picsum.photos)\" /><figcaption>This contains a <a href=\"https://picsum.photos\">link</a></figcaption></figure>",
    );
}

#[test]
fn code_blocks() {
    test_render("```", "<p>```</p>");
    let theme = include_str!("../../../../theme.css").parse().unwrap();
    let mut hl = Highlight::new(theme);
    let python = HighlightConfiguration::new(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        tree_sitter_python::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap();
    hl.add_language(&["python", "py"], python);

    let highlighted = hl.highlight("print(\"Your code here\")".to_string(), "py".to_string());
    test_render(
        r#"
```py
print("Your code here")
```
"#
        .to_string(),
        highlighted,
    );

    let highlighted = hl.highlight(
        "This is some text in a code block".to_string(),
        "plaintext".to_string(),
    );
    test_render(
        r#"
```
This is some text in a code block
```
"#
        .to_string(),
        highlighted,
    );

    let highlighted = hl.highlight(
        "This is for a language that doesn't exist".to_string(),
        "plaintext".to_string(),
    );
    test_render(
        r#"
```skajdlas
This is for a language that doesn't exist
```
"#
        .to_string(),
        highlighted,
    );
}

#[test]
fn skip_frontmatter() {
    test_render(
        r#"
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
"#,
        "<hr /><p>I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.</p>",
    );
}

#[test]
fn inline_formatting() {
    test_render("*bold text*", "<p><strong>bold text</strong></p>");
    test_render("_italic text_", "<p><em>italic text</em></p>");
    test_render(
        "_*bold and italic text*_",
        "<p><em><strong>bold and italic text</strong></em></p>",
    );
    test_render(
        "*_bold and italic text_*",
        "<p><strong><em>bold and italic text</em></strong></p>",
    );
    test_render(
        "This is some `inline code`",
        "<p>This is some <code>inline code</code></p>",
    );
    test_render(
        "This is a [link](https://zerolimits.dev)",
        "<p>This is a <a href=\"https://zerolimits.dev\">link</a></p>",
    );
    test_render(
        "This is a *[bold link](https://zerolimits.dev)*",
        "<p>This is a <strong><a href=\"https://zerolimits.dev\">bold link</a></strong></p>",
    );
    test_render(
        "This is a [*bold link*](https://zerolimits.dev)",
        "<p>This is a <a href=\"https://zerolimits.dev\"><strong>bold link</strong></a></p>",
    );
    test_render(
        "This is an _[italic link](https://zerolimits.dev)_",
        "<p>This is an <em><a href=\"https://zerolimits.dev\">italic link</a></em></p>",
    );
    test_render(
        "This is an [_italic link_](https://zerolimits.dev)",
        "<p>This is an <a href=\"https://zerolimits.dev\"><em>italic link</em></a></p>",
    );
    test_render(
        "This is a `[code link](https://zerolimits.dev)`",
        "<p>This is a <code>[code link](https://zerolimits.dev)</code></p>",
    );
    test_render(
        "This is a [`code link`](https://zerolimits.dev)",
        "<p>This is a <a href=\"https://zerolimits.dev\"><code>code link</code></a></p>",
    );
    test_render(
        "This is formatting inside a `*code* _block_`",
        "<p>This is formatting inside a <code>*code* _block_</code></p>",
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
        "<p><a href=\"https://en.wikipedia.org/wiki/Rust_(programming_language)\">link with parentheses</a></p>",
    );
    test_render(
        "[Some [square braces] inside a link](https://zerolimits.dev)",
        "<p><a href=\"https://zerolimits.dev\">Some [square braces] inside a link</a></p>",
    );
    test_render(
        "Inline $x+y$ math",
        "<p>Inline <math><mi>x</mi><mo>+</mo><mi>y</mi></math> math</p>",
    );
    test_render(
        "This is a *line* with multiple *bold* words",
        "<p>This is a <strong>line</strong> with multiple <strong>bold</strong> words</p>",
    );
    test_render(
        "Special characters: “_voilà_!”",
        "<p>Special characters: “<em>voilà</em>!”</p>",
    );
    test_render(
        "This website is a memorial to him, and hosts [his story on Boško Brkić and Admira Ismić](http://www.ksmemorial.com/romeo.htm)",
        "<p>This website is a memorial to him, and hosts <a href=\"http://www.ksmemorial.com/romeo.htm\">his story on Boško Brkić and Admira Ismić</a></p>",
    );
}

#[test]
fn escaped_characters() {
    test_render("This is \\*escaped bold*", "<p>This is *escaped bold*</p>");
}

#[test]
fn empty_inline() {
    test_render("**", "<p>**</p>");
    test_render("__", "<p>__</p>");
    test_render("$$", "<p>$$</p>");
    test_render(
        "[](https://zerolimits.dev)",
        "<p>[](https://zerolimits.dev)</p>",
    );
    test_render("[link]()", "<p>[link]()</p>");
    test_render("^^", "<p>^^</p>");
    test_render("~~", "<p>~~</p>");
}

#[test]
fn lists() {
    test_render(
        r#"
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1
   2. nested list item 2
      1. You can nest as far as you want
"#,
        "<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
    );
    test_render(
        r#"
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1
  - nested list item 2
    - You can nest as far as you want
"#,
        "<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
    );

    let theme = include_str!("../../../../theme.css").parse().unwrap();
    let hl = Highlight::new(theme);
    let code_block = hl.highlight("code block".to_string(), "plaintext".to_string());
    test_render(
        r#"
1. Repeat steps 2-4 until you reach the beginning of the array

\[...]

```
code block
```
"#,
        format!(
            r#"<ol><li><p>Repeat steps 2-4 until you reach the beginning of the array</p></li></ol><p>[...]</p>{}"#,
            code_block
        ),
    );
}

#[test]
fn tables() {
    test_render(
        r#"
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
"#,
        "<table><thead><tr><th align=\"left\">title</th><th align=\"center\">description</th><th align=\"right\">heading 1</th><th align=\"\">heading 2</th></tr></thead><tbody><tr><td align=\"left\">left-aligned</td><td align=\"center\">center-aligned</td><td align=\"right\">right-aligned</td><td align=\"\">default text alignment</td></tr></tbody></table>",
    );
}

#[test]
fn html_elements() {
    test_render(
        r#"
<div>
Content here
</div>
"#,
        r#"<div>
Content here
</div>"#,
    );
    test_render(
        "<u>This element isn't closed",
        "<u>This element isn't closed",
    );
    test_render(
        r#"
<div>
  <div class="nested">
    <p>Some content here</p>
  </div>
</div>
"#,
        r#"<div>
  <div class="nested">
    <p>Some content here</p>
  </div>
</div>"#,
    );
}

#[test]
fn math_block() {
    test_render(
        r#"
$$
a^2 + b^2 = c^2
$$
"#,
        "<math display=\"block\"><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></math>",
    );
    test_render("$$", "<p>$$</p>");
}

#[test]
fn containers() {
    test_render(":::", "<p>:::</p>");
    test_render(
        r#"
::: note A NOTE
This is some text in a note.
:::
"#,
        "<div class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>",
    );
    test_render(
        r#"
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
"#,
        "<div class=\"znak-container quote\"><p class=\"quote-heading\"><b><a href=\"https://zerolimits.dev\">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>",
    );
    test_render(
        r#"
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
"#,
        "<div class=\"znak-container quote bold\"><p class=\"quote-heading\"><b><a href=\"https://zerolimits.dev\">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>",
    );
    test_render(
        r#"
::: warning
This is some text in a warning.
:::
"#,
        "<div class=\"znak-container warning\"><p class=\"warning-heading\"><b>WARNING</b></p><p>This is some text in a warning.</p></div>",
    );
    test_render(
        r#"
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
"#,
        "<div class=\"znak-container block1\"><p class=\"block1-heading\"><b>This is the outer container</b></p><p>You can have some text here.</p><div class=\"znak-container block2\"><p class=\"block2-heading\"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>",
    );
    test_render(
        r#"
::: note A NOTE {id="my-note"}
This is some text in a note.
:::
"#,
        "<div id=\"my-note\" class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>",
    );
    test_render(
        r#"
::: note A NOTE
This is some text in a note.
:::

::: warning
This is some text in a warning.
:::
"#,
        "<div class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div><div class=\"znak-container warning\"><p class=\"warning-heading\"><b>WARNING</b></p><p>This is some text in a warning.</p></div>",
    );
}
