use crate::types::{Node, comment, doctype, element, root, text};

#[test]
fn doctype() {
    let want = root!([doctype!()]);
    let got = "<!doctype html>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([doctype!()]);
    let got = " \n <!DOCTYPE html>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([
        doctype!(),
        doctype!(),
        doctype!(),
        text!("text before "),
        doctype!(),
        text!("text after\n"),
    ]);
    let got = "<!DOCTYPE html>
<!DoCtYpE html>
<!doctype HTML>
text before <!doctype html> text after
"
    .parse()
    .unwrap();
    assert_eq!(want, got);

    let want = root!([
        doctype!(),
        element!("html", [element!("body", [text!("x")])]),
    ]);
    let got = "<!doctype html><html><body>x</body></html>"
        .parse()
        .unwrap();
    assert_eq!(want, got);
}

#[test]
fn comment() {
    let want = root!([doctype!(), comment!(" This is a comment ")]);
    let got = "
            <!doctype html>
            <!-- This is a comment -->
    "
    .parse()
    .unwrap();
    assert_eq!(want, got);

    let want = root!([comment!(" This is a comment ")]);
    let got = "<!-- This is a comment -->".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([comment!("hi")]);
    let got = "<!--hi-->".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([comment!(" I love cookies 🍪 ")]);
    let got = "<!-- I love cookies 🍪 -->".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([text!("a"), comment!(" I love cookies 🍪 "), text!("b"),]);
    let got = "a<!-- I love cookies 🍪 -->b".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([comment!("unterminated comment")]);
    let got = "<!--unterminated comment".parse().unwrap();
    assert_eq!(want, got);
}

#[test]
fn text() {
    let want = root!([text!("This is some text")]);
    let got = "This is some text".parse().unwrap();
    assert_eq!(want, got);
}

#[test]
fn element() {
    let want = root!([element!(
        "h1",
        {"id" => "heading-1"},
        [text!("Heading 1")]
    )]);
    let got = "<h1 id=\"heading-1\">Heading 1</h1>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([
        element!("h2",{"id" => "heading"},[text!("Heading")]),
        element!("h3",{"id" => "heading-1"},[text!("Heading")]),
        element!("h4",{"id" => "heading-2"},[text!("Heading")]),
    ]);
    let got =
        r#"<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>"#.parse().unwrap();
    assert_eq!(want, got);

    let want = root!([element!(
        "blockquote",
        [element!(
            "p",
            [
                text!("This is quite a "),
                element!("strong", [text!("bold")]),
                text!(" statement!")
            ]
        )]
    )]);
    let got = "<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>"
        .parse()
        .unwrap();
    assert_eq!(want, got);

    let want = root!([element!(
        "blockquote",
        [
            element!("p", [text!("This is a")]),
            element!("p", [text!("multiline blockquote")])
        ]
    )]);
    let got = "<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>"
        .parse()
        .unwrap();
    assert_eq!(want, got);

    let want = root!([element!("hr")]);
    let got = "<hr />".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([element!(
        "figure",
        [
            element!("img", {
                "src" => "https://picsum.photos/300",
                "alt" => "This contains a [link](https://picsum.photos)"
            }),
            element!(
                "figcaption",
                [
                    text!("This contains a "),
                    element!("a", {"href" => "https://picsum.photos"}, [text!("link")])
                ]
            )
        ]
    )]);
    let got =
        "<figure><img src=\"https://picsum.photos/300\" alt=\"This contains a [link](https://picsum.photos)\" /><figcaption>This contains a <a href=\"https://picsum.photos\">link</a></figcaption></figure>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([element!("div", {"class" => "znak-container note"}, [
        element!("p", {"class" => "note-heading"}, [element!("b", [text!("A NOTE")])]),
        element!("p", [text!("This is some text in a note.")])
    ])]);
    let got =
        "<div class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([element!("div",
            {"class" => "znak-container block1"},
            [
                element!("p",
                    {"class" => "block1-heading"},
                    [
                        element!("b", [text!("This is the outer container")])
                    ]
                )
                element!("p", [text!("You can have some text here.")]),
                element!("div", {"class" => "znak-container block2"}, [
                    element!("p", {"class" => "block2-heading"}, [
                        element!("b", [text!("This is the inner container")])
                    ]),
                    element!("p", [text!("This can have some more text.")])
                ])
            ]
    )]);
    let got =
        "<div class=\"znak-container block1\"><p class=\"block1-heading\"><b>This is the outer container</b></p><p>You can have some text here.</p><div class=\"znak-container block2\"><p class=\"block2-heading\"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([
        element!("div", {"id" => "my-note", "class" => "znak-container note"}, [
            element!("p", {"class" => "note-heading"}, [
                element!("b", [text!("A NOTE")])
            ]),
            element!("p", [text!("This is some text in a note.")])
        ])
    ]);
    let got =
        "<div id=\"my-note\" class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>".parse().unwrap();
    assert_eq!(want, got);

    let want = root!([element!(
        "div",
        [element!("div", {"class" => "nested"}, [
            element!("p", [text!("Some content here")])
        ])]
    )]);
    let got = r#"<div>
<div class="nested">
<p>Some content here</p>
</div>
</div>"#
        .parse()
        .unwrap();
    assert_eq!(want, got);

    let want = root!([
        element!("link", {"rel" => "stylesheet", "href" => "/styles/style.css"}),
        element!("link", {"rel" => "alternate", "href" => "/feed.atom"})
    ]);
    let got = r#"
        <link rel="stylesheet" href="/styles/style.css">
        <link rel="alternate" href="/feed.atom">"#
        .parse()
        .unwrap();
    assert_eq!(want, got);

    let want = root!([element!("code", {"data-lang" => "json"}, [
        text!("{"),
        element!("br"),
        text!(r#"  "ip": "[REDACTED]","#),
        element!("br"),
        text!(r#"  "count": 10,"#),
        element!("br"),
        text!(r#"  "limit": 10,"#),
        element!("br"),
        text!(r#"  "remaining": 0,"#),
        element!("br")
        text!(r#"  "resetAt": "2025-06-23T17:17:24.201Z""#),
        element!("br"),
        text!("}")
    ])]);
    let got = r#"<code data-lang="json">{<br>  "ip": "[REDACTED]",<br>  "count": 10,<br>  "limit": 10,<br>  "remaining": 0,<br>  "resetAt": "2025-06-23T17:17:24.201Z"<br>}</code>"#.parse().unwrap();
    assert_eq!(want, got)
}
