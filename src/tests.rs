use crate::{compile, frontmatter};

fn render(source: &str) -> String {
    compile(source).unwrap()
}

#[test]
fn fm() {
    assert_eq!(
        frontmatter(
            "\
---
title = A title
---"
        )
        .unwrap(),
        vec![("title".to_string(), "A title".to_string())]
    );
    assert_eq!(
        frontmatter(
            "\
---
title = A title with = equal signs
---"
        )
        .unwrap(),
        vec![(
            "title".to_string(),
            "A title with = equal signs".to_string()
        )]
    );
    assert_eq!(
        frontmatter(
            "\
---
title = A title
description = A description
date = 2026-10-07
---"
        )
        .unwrap(),
        vec![
            ("title".to_string(), "A title".to_string()),
            ("description".to_string(), "A description".to_string()),
            ("date".to_string(), "2026-10-07".to_string())
        ]
    );
    assert_eq!(
        frontmatter(
            "\
---
key = value
title = a title that contains = equal signs
---

Content here"
        )
        .unwrap(),
        vec![
            ("key".to_string(), "value".to_string()),
            (
                "title".to_string(),
                "a title that contains = equal signs".to_string()
            )
        ]
    );
    assert_eq!(
        render(
            "\
---
key = value
title = a title that contains = equal signs
---

Content here"
        ),
        "<p>Content here</p>"
    )
}

#[test]
fn headings() {
    assert_eq!(
        render("= Heading 1"),
        r#"<h1 id="heading-1">Heading 1</h1>"#
    );
    assert_eq!(
        render("== Heading 2"),
        r#"<h2 id="heading-2">Heading 2</h2>"#
    );
    assert_eq!(
        render("=== Heading 3"),
        r#"<h3 id="heading-3">Heading 3</h3>"#
    );
    assert_eq!(
        render("==== Heading 4"),
        r#"<h4 id="heading-4">Heading 4</h4>"#
    );
    assert_eq!(
        render("===== Heading 5"),
        r#"<h5 id="heading-5">Heading 5</h5>"#
    );
    assert_eq!(
        render("====== Heading 6"),
        r#"<h6 id="heading-6">Heading 6</h6>"#
    );
    assert_eq!(
        render(
            "\
= Heading

== Heading
"
        ),
        r#"<h1 id="heading">Heading</h1><h2 id="heading-1">Heading</h2>"#
    );
    assert_eq!(
        render(
            "\
= heading 1

== heading 2

=== heading 3

==== heading 4

===== heading 5

====== heading 6"
        ),
        r#"<h1 id="heading-1">heading 1</h1><h2 id="heading-2">heading 2</h2><h3 id="heading-3">heading 3</h3><h4 id="heading-4">heading 4</h4><h5 id="heading-5">heading 5</h5><h6 id="heading-6">heading 6</h6>"#
    );
}

#[test]
fn blockquotes() {
    assert_eq!(
        render(
            "\
> This is a blockquote"
        ),
        "<blockquote><p>This is a blockquote</p></blockquote>"
    );
    assert_eq!(
        render(
            "\
> This is a blockquote
>
> == Heading
>
>-- Attribution"
        ),
        r#"<blockquote><p>This is a blockquote</p><h2 id="heading">Heading</h2></blockquote><p>— Attribution</p>"#
    );
    assert_eq!(
        render(
            "\
> blockquote content
>
> = heading in blockquote
>
>-- blockquote attribution
>-- over multiple lines
>-- combined into one"
        ),
        r#"<blockquote><p>blockquote content</p><h1 id="heading-in-blockquote">heading in blockquote</h1></blockquote><p>— blockquote attribution over multiple lines combined into one</p>"#
    )
}

#[test]
fn unordered_list() {
    assert_eq!(
        render(
            "\
- unordered list item
- list item
  - 2 space indentation
  - nested list item
    + nested ordered list item"
        ),
        "<ul><li><p>unordered list item</p></li><li><p>list item</p><ul><li><p>2 space indentation</p></li><li><p>nested list item</p><ol><li><p>nested ordered list item</p></li></ol></li></ul></li></ul>"
    );
}

#[test]
fn ordered_list() {
    assert_eq!(
        render(
            "\
+ ordered list item
+ list item
  + 2 space indentation
  + nested list item
    - nested unordered list item"
        ),
        "<ol><li><p>ordered list item</p></li><li><p>list item</p><ol><li><p>2 space indentation</p></li><li><p>nested list item</p><ul><li><p>nested unordered list item</p></li></ul></li></ol></li></ol>"
    );
}

#[test]
fn code_blocks() {
    assert_eq!(
        render(
            "\
```
Code here
```

````znak
Nested code blocks with additional backticks

```py
print(\"nested hello world\")
```
````"
        ),
        r#"<pre><code>Code here</code></pre><pre data-lang="znak"><code>Nested code blocks with additional backticks

```py
print("nested hello world")
```</code></pre>"#
    );
}

#[test]
fn containers() {
    assert_eq!(
        render(
            "\
::: name [Inline markup]<https://example.org>
Normal block content here.

= Including headings
:::
"
        ),
        r#"<div class="container name"><p class="container-heading"><a href="https://example.org">Inline markup</a></p><p>Normal block content here.</p><h1 id="including-headings">Including headings</h1></div>"#
    );
    assert_eq!(
        render(
            "\
:::: block1 A title
A container with 4 colons

::: block2
A nested container
:::
::::
"
        ),
        r#"<div class="container block1"><p class="container-heading">A title</p><p>A container with 4 colons</p><div class="container block2"><p class="container-heading">BLOCK2</p><p>A nested container</p></div></div>"#
    )
}

#[test]
fn tables() {
    assert_eq!(
        render(
            "\
| Table | header | here  |
| ----- | ------ | ----- |
| Table | body   | here  |
| left  | center | right |
| ----- | ------ | ----- |
| Table | footer | here  |"
        ),
        r#"<table><thead><tr><th><p>Table</p></th><th><p>header</p></th><th><p>here</p></th></tr></thead><tbody><tr><td><p>Table</p></td><td><p>body</p></td><td><p>here</p></td></tr><tr><td><p>left</p></td><td><p>center</p></td><td><p>right</p></td></tr></tbody><tfoot><tr><td><p>Table</p></td><td><p>footer</p></td><td><p>here</p></td></tr></tfoot></table>"#
    );
    assert_eq!(
        render(
            "\
| Table | header | here  |
| ----- | ------ | ----- |
| Table | body   | here  |
| left  | center | right |
| ----- | ------ | ----- |
| Table | footer | here  |
| ----- | ------ | ----- |
| some  | lines  | above |"
        ),
        r#"<table><thead><tr><th><p>Table</p></th><th><p>header</p></th><th><p>here</p></th></tr></thead><tbody><tr><td><p>Table</p></td><td><p>body</p></td><td><p>here</p></td></tr><tr><td><p>left</p></td><td><p>center</p></td><td><p>right</p></td></tr></tbody><tfoot><tr><td><p>Table</p></td><td><p>footer</p></td><td><p>here</p></td></tr><tr><td><hr></td><td><hr></td><td><hr></td></tr><tr><td><p>some</p></td><td><p>lines</p></td><td><p>above</p></td></tr></tfoot></table>"#
    )
}

#[test]
fn images() {
    assert_eq!(
        render("![Image with caption]<https://example.org/image.png>"),
        r#"<img src="https://example.org/image.png" alt="Image with caption">"#
    );
    assert_eq!(
        render("![Image with caption and [brackets]]<https://example.org/image.png>"),
        r#"<img src="https://example.org/image.png" alt="Image with caption and [brackets]">"#
    );
    assert_eq!(
        render("![]<https://example.org/image.png>"),
        r#"<img src="https://example.org/image.png">"#
    );
}

#[test]
fn inline_formatting() {
    assert_eq!(
        render(r"Some text \ with a break"),
        "<p>Some text <br>with a break</p>"
    );
    assert_eq!(render(r"An \*escaped bold\*"), "<p>An *escaped bold*</p>");
    assert_eq!(render(r"*bold*"), "<p><strong>bold</strong></p>");
    assert_eq!(render(r"/italics/"), "<p><em>italics</em></p>");
    assert_eq!(render(r"~strikethrough~"), "<p><s>strikethrough</s></p>");
    assert_eq!(render(r"_subscript_"), "<p><sub>subscript</sub></p>");
    assert_eq!(render(r"^superscript^"), "<p><sup>superscript</sup></p>");
    assert_eq!(render(r"=highlight="), "<p><mark>highlight</mark></p>");
    assert_eq!(render(r"`code`"), "<p><code>code</code></p>");
    assert_eq!(
        render(r"`code with *formatting*`"),
        "<p><code>code with *formatting*</code></p>"
    );
    assert_eq!(
        render(r"``py print('hello world')``"),
        r#"<p><code data-lang="py">print('hello world')</code></p>"#
    );
    assert_eq!(
        render(r"[a link]<https://example.org>"),
        r#"<p><a href="https://example.org">a link</a></p>"#
    );
    assert_eq!(
        render("<https://example.org/a/bare/link>"),
        r#"<p><a href="https://example.org/a/bare/link">https://example.org/a/bare/link</a></p>"#
    );
    assert_eq!(
        render("$x+y$"),
        r#"<p><span class="math inline">x+y</span></p>"#
    );
    assert_eq!(
        render(
            r#"This is a sentence with *bold*, /italics/, ~strikethrough~, _subscript_, ^superscript^, =highlight=, `inline code`, ``py print("highlighted inline code")``, [a link]<https://example.org>, <https://example.org/a/bare/link>, and inline $x+y$ math."#
        ),
        r#"<p>This is a sentence with <strong>bold</strong>, <em>italics</em>, <s>strikethrough</s>, <sub>subscript</sub>, <sup>superscript</sup>, <mark>highlight</mark>, <code>inline code</code>, <code data-lang="py">print("highlighted inline code")</code>, <a href="https://example.org">a link</a>, <a href="https://example.org/a/bare/link">https://example.org/a/bare/link</a>, and inline <span class="math inline">x+y</span> math.</p>"#
    );
    assert_eq!(
        render(
            r#"This is a sentence with \*bold\*, \/italics\/, \~strikethrough\~, \_subscript\_, \^superscript\^, \=highlight\=, \`inline code\`, \`\`py print("highlighted inline code")\`\`, \[a link]\<https:\/\/example.org>, \<https:\/\/example.org\/a\/bare\/link>, and inline \$x+y\$ math."#
        ),
        r#"<p>This is a sentence with *bold*, /italics/, ~strikethrough~, _subscript_, ^superscript^, =highlight=, `inline code`, ``py print("highlighted inline code")``, [a link]<https://example.org>, <https://example.org/a/bare/link>, and inline $x+y$ math.</p>"#
    )
}

#[test]
fn raw() {
    assert_eq!(
        render(
            "\
{{
<figure>
}}"
        ),
        "<figure>"
    );
    assert_eq!(
        render("This is a sentence with a {{<raw>}} block"),
        "<p>This is a sentence with a <raw> block</p>"
    )
}

#[test]
fn paragraphs() {
    assert_eq!(
        render(
            "\
This is a paragraph.
This is a second line of this paragraph.

This is another paragraph.
"
        ),
        "<p>This is a paragraph. This is a second line of this paragraph.</p><p>This is another paragraph.</p>"
    )
}
