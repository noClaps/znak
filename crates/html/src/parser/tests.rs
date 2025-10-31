use crate::parser::{
    parse,
    types::{Node, comment, doctype, element, root, text},
};

#[test]
fn doctype() {
    let want = root!([doctype!()]);
    let got = parse("<!doctype html>");
    assert_eq!(want, got);

    let want = root!([doctype!()]);
    let got = parse(" \n <!DOCTYPE html>");
    assert_eq!(want, got);

    let want = root!([
        doctype!(),
        doctype!(),
        doctype!(),
        text!("text before "),
        doctype!(),
        text!("text after\n"),
    ]);
    let got = parse(
        "<!DOCTYPE html>
<!DoCtYpE html>
<!doctype HTML>
text before <!doctype html> text after
",
    );
    assert_eq!(want, got);

    let want = root!([
        doctype!(),
        element!("html", [element!("body", [text!("x")])]),
    ]);
    let got = parse("<!doctype html><html><body>x</body></html>");
    assert_eq!(want, got);
}

#[test]
fn comment() {
    let want = root!([doctype!(), comment!("This is a comment")]);
    let got = parse(
        "
            <!doctype html>
            <!-- This is a comment -->
    ",
    );
    assert_eq!(want, got);

    let want = root!([comment!("This is a comment")]);
    let got = parse("<!-- This is a comment -->");
    assert_eq!(want, got);

    let want = root!([comment!("hi")]);
    let got = parse("<!--hi-->");
    assert_eq!(want, got);

    let want = root!([comment!("I love cookies 🍪")]);
    let got = parse("<!-- I love cookies 🍪 -->");
    assert_eq!(want, got);

    let want = root!([text!("a"), comment!("I love cookies 🍪"), text!("b"),]);
    let got = parse("a<!-- I love cookies 🍪 -->b");
    assert_eq!(want, got);

    let want = root!([comment!("unterminated comment")]);
    let got = parse("<!--unterminated comment");
    assert_eq!(want, got);
}

#[test]
fn text() {
    let want = root!([text!("This is some text")]);
    let got = parse("This is some text");
    assert_eq!(want, got);
}

#[test]
fn element() {
    let want = root!([element!(
        "h1",
        {id: "heading-1"},
        [text!("Heading 1")]
    )]);
    let got = parse("<h1 id=\"heading-1\">Heading 1</h1>");
    assert_eq!(want, got);

    let want = root!([
        element!("h2",{id: "heading"},[text!("Heading")]),
        element!("h3",{id: "heading-1"},[text!("Heading")]),
        element!("h4",{id: "heading-2"},[text!("Heading")]),
    ]);
    let got = parse(
        r#"<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>"#,
    );
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
    let got =
        parse("<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>");
    assert_eq!(want, got);

    let want = root!([element!(
        "blockquote",
        [
            element!("p", [text!("This is a")]),
            element!("p", [text!("multiline blockquote")])
        ]
    )]);
    let got = parse("<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>");
    assert_eq!(want, got);

    let want = root!([element!("hr")]);
    let got = parse("<hr />");
    assert_eq!(want, got);

    let want = root!([element!(
        "figure",
        [
            element!("img", {
                src: "https://picsum.photos/300",
                alt: "This contains a [link](https://picsum.photos)"
            }),
            element!(
                "figcaption",
                [
                    text!("This contains a "),
                    element!("a", {href: "https://picsum.photos"}, [text!("link")])
                ]
            )
        ]
    )]);
    let got = parse(
        "<figure><img src=\"https://picsum.photos/300\" alt=\"This contains a [link](https://picsum.photos)\" /><figcaption>This contains a <a href=\"https://picsum.photos\">link</a></figcaption></figure>",
    );
    assert_eq!(want, got);

    let want = root!([element!("div", {class: "znak-container note"}, [
        element!("p", {class: "note-heading"}, [element!("b", [text!("A NOTE")])]),
        element!("p", [text!("This is some text in a note.")])
    ])]);
    let got = parse(
        "<div class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>",
    );
    assert_eq!(want, got);

    let want = root!([element!("div",
            {class: "znak-container block1"},
            [
                element!("p",
                    {class: "block1-heading"},
                    [
                        element!("b", [text!("This is the outer container")])
                    ]
                )
                element!("p", [text!("You can have some text here.")]),
                element!("div", {class: "znak-container block2"}, [
                    element!("p", {class: "block2-heading"}, [
                        element!("b", [text!("This is the inner container")])
                    ]),
                    element!("p", [text!("This can have some more text.")])
                ])
            ]
    )]);
    let got = parse(
        "<div class=\"znak-container block1\"><p class=\"block1-heading\"><b>This is the outer container</b></p><p>You can have some text here.</p><div class=\"znak-container block2\"><p class=\"block2-heading\"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>",
    );
    assert_eq!(want, got);

    let want = root!([
        element!("div", {id: "my-note" class: "znak-container note"}, [
            element!("p", {class: "note-heading"}, [
                element!("b", [text!("A NOTE")])
            ]),
            element!("p", [text!("This is some text in a note.")])
        ])
    ]);
    let got = parse(
        "<div id=\"my-note\" class=\"znak-container note\"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>",
    );
    assert_eq!(want, got);

    let want = root!([element!(
        "div",
        [element!("div", {class: "nested"}, [
            element!("p", [text!("Some content here")])
        ])]
    )]);
    let got = parse(
        r#"<div>
<div class="nested">
<p>Some content here</p>
</div>
</div>"#,
    );
    assert_eq!(want, got);
}
