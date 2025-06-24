package parser_test

import (
	"os"
	"slices"
	"testing"

	"github.com/noClaps/znak"
	"github.com/noClaps/znak/highlight"
)

func testRender(input string, test string, t *testing.T) {
	themeFile, err := os.ReadFile("../../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}
	output, err := znak.Render(input, theme)
	if err != nil {
		t.Fatal(err)
	}
	if output != test {
		t.Fatalf("Rendering failed.\nInput: %s\nExpected: %s\nReceived: %s\n", input, test, output)
	}
}

func testRenderNe(input string, test string, t *testing.T) {
	themeFile, err := os.ReadFile("../../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}
	output, err := znak.Render(input, theme)
	if err != nil {
		t.Fatal(err)
	}
	if output == test {
		t.Fatalf("Rendering failed.\nInput: %s\nExpected: %s\nReceived: %s\n", input, test, output)
	}
}

func testRenderOneOf(input string, tests []string, t *testing.T) {
	themeFile, err := os.ReadFile("../../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}
	output, err := znak.Render(input, theme)
	if err != nil {
		t.Fatal(err)
	}
	if slices.Contains(tests, output) {
		return
	}
	t.Fatalf("Rendering failed.\nInput: %s\nExpected: %s\nReceived: %s\n", input, tests, output)
}

func TestHeadings(t *testing.T) {
	testRender("# Heading 1", `<h1 id="heading-1">Heading 1</h1>`, t)
	testRender("## Heading 2", "<h2 id=\"heading-2\">Heading 2</h2>", t)
	testRender("### Heading 3", "<h3 id=\"heading-3\">Heading 3</h3>", t)
	testRender("#### Heading 4", "<h4 id=\"heading-4\">Heading 4</h4>", t)
	testRender("##### Heading 5", "<h5 id=\"heading-5\">Heading 5</h5>", t)
	testRender("###### Heading 6", "<h6 id=\"heading-6\">Heading 6</h6>", t)
	testRender(`
## Heading
### Heading
#### Heading
`, `<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>`, t)
}

func TestHorizontalRule(t *testing.T) {
	testRender("---", "<hr />", t)
	testRender("-----", "<hr />", t)
	testRenderNe("-------asdsa", "<hr />", t)
}

func TestInlineFormatting(t *testing.T) {
	testRender("**bold text**", "<p><strong>bold text</strong></p>", t)
	testRender("_italic text_", "<p><em>italic text</em></p>", t)
	testRender("_**bold and italic text**_", "<p><em><strong>bold and italic text</strong></em></p>", t)
	testRender("**_bold and italic text_**", "<p><strong><em>bold and italic text</em></strong></p>", t)
	testRender("This is some `inline code`", "<p>This is some <code>inline code</code></p>", t)
	testRender("This is a [link](https://zerolimits.dev)", `<p>This is a <a href="https://zerolimits.dev">link</a></p>`, t)
	testRender(
		"This is a **[bold link](https://zerolimits.dev)**",
		`<p>This is a <strong><a href="https://zerolimits.dev">bold link</a></strong></p>`,
		t,
	)
	testRender(
		"This is a [**bold link**](https://zerolimits.dev)",
		`<p>This is a <a href="https://zerolimits.dev"><strong>bold link</strong></a></p>`,
		t,
	)
	testRender(
		"This is an _[italic link](https://zerolimits.dev)_",
		`<p>This is an <em><a href="https://zerolimits.dev">italic link</a></em></p>`,
		t,
	)
	testRender(
		"This is an [_italic link_](https://zerolimits.dev)",
		`<p>This is an <a href="https://zerolimits.dev"><em>italic link</em></a></p>`,
		t,
	)
	testRender(
		"This is a `[code link](https://zerolimits.dev)`",
		`<p>This is a <code>[code link](https://zerolimits.dev)</code></p>`,
		t,
	)
	testRender(
		"This is a [`code link`](https://zerolimits.dev)",
		`<p>This is a <a href="https://zerolimits.dev"><code>code link</code></a></p>`,
		t,
	)
	testRender(
		"This is formatting inside a `**code** _block_`",
		"<p>This is formatting inside a <code>**code** _block_</code></p>",
		t,
	)
	testRender("~~strikethrough~~", "<p><s>strikethrough</s></p>", t)
	testRender(
		"This is a sentence with ~~strikethrough~~ in it",
		"<p>This is a sentence with <s>strikethrough</s> in it</p>",
		t,
	)
	testRender("==highlight==", "<p><mark>highlight</mark></p>", t)
	testRender(
		"This is a sentence with ==highlight== in it",
		"<p>This is a sentence with <mark>highlight</mark> in it</p>",
		t,
	)
	testRender("~sub~script", "<p><sub>sub</sub>script</p>", t)
	testRender(
		"This is a sentence with ~sub~script in it",
		"<p>This is a sentence with <sub>sub</sub>script in it</p>",
		t,
	)
	testRender("^super^script", "<p><sup>super</sup>script</p>", t)
	testRender(
		"This is a sentence with ^super^script in it",
		"<p>This is a sentence with <sup>super</sup>script in it</p>",
		t,
	)
	testRender(
		"[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)",
		`<p><a href="https://en.wikipedia.org/wiki/Rust_(programming_language)">link with parentheses</a></p>`,
		t,
	)
	testRender(
		"[Some [square braces] inside a link](https://zerolimits.dev)",
		`<p><a href="https://zerolimits.dev">Some [square braces] inside a link</a></p>`,
		t,
	)
	testRender(
		"Inline $$x+y$$ math",
		`<p>Inline <math display="inline"><semantics><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><annotation encoding="application/x-tex">x+y</annotation></semantics></math> math</p>`,
		t,
	)
	testRender(
		"This is a **line** with multiple **bold** words",
		"<p>This is a <strong>line</strong> with multiple <strong>bold</strong> words</p>",
		t,
	)
	testRender("This is some __underlined__ text", "<p>This is some <u>underlined</u> text</p>", t)
	testRender(
		"This is some ___italic text_ in some underlined__ text",
		"<p>This is some <u><em>italic text</em> in some underlined</u> text</p>",
		t,
	)
	testRender("Special characters: “_voilà_!”", "<p>Special characters: “<em>voilà</em>!”</p>", t)
}

func TestBlockquotes(t *testing.T) {
	testRender("> This is text in a blockquote", "<blockquote><p>This is text in a blockquote</p></blockquote>", t)
	testRender(`
> This is a
>
> multiline blockquote
`,
		"<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>",
		t,
	)
}

func TestImages(t *testing.T) {
	testRenderOneOf(
		"![alt text](https://picsum.photos/300)",
		[]string{
			`<figure><img src="https://picsum.photos/300" alt="alt text" /><figcaption>alt text</figcaption></figure>`,
			`<figure><img alt="alt text" src="https://picsum.photos/300" /><figcaption>alt text</figcaption></figure>`,
		},
		t,
	)
	testRenderOneOf(
		"![This contains a [link](https://picsum.photos)](https://picsum.photos/300)",
		[]string{
			`<figure><img src="https://picsum.photos/300" alt="This contains a [link](https://picsum.photos)" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>`,
			`<figure><img alt="This contains a [link](https://picsum.photos)" src="https://picsum.photos/300" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>`,
		},
		t,
	)
}

func TestCodeBlocks(t *testing.T) {
	themeFile, err := os.ReadFile("../../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}

	highlighted, err := highlight.Highlight(`print("Your code here")`, "py", theme)
	if err != nil {
		t.Fatal(err)
	}
	testRender("```py\n"+
		"print(\"Your code here\")\n"+
		"```\n", highlighted, t)

	highlighted, err = highlight.Highlight("This is some text in a code block", "plaintext", theme)
	if err != nil {
		t.Fatal(err)
	}
	testRender("```\n"+
		"This is some text in a code block\n"+
		"```\n", highlighted, t)

	highlighted, err = highlight.Highlight("This is for a language that doesn't exist", "plaintext", theme)
	if err != nil {
		t.Fatal(err)
	}
	testRender(
		"```skajdlas\n"+
			"This is for a language that doesn't exist\n"+
			"```\n", highlighted, t)
}

func TestLists(t *testing.T) {
	testRender(`
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1
   2. nested list item 2
      1. You can nest as far as you want
`,
		"<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
		t,
	)
	testRender(`
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1
  - nested list item 2
    - You can nest as far as you want
`,

		"<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
		t,
	)
}

func TestTables(t *testing.T) {
	testRender(`
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
`,
		`<table><thead><tr><th align="left">title</th><th align="center">description</th><th align="right">heading 1</th><th align="">heading 2</th></tr></thead><tbody><tr><td align="left">left-aligned</td><td align="center">center-aligned</td><td align="right">right-aligned</td><td align="">default text alignment</td></tr></tbody></table>`,
		t,
	)
}

func TestHtmlElements(t *testing.T) {
	testRender(`
<div>
Content here
</div>
`,
		`<div>
Content here
</div>`,
		t,
	)
	testRender("<u>This element isn't closed", "<u>This element isn't closed", t)
}

func TestMathBlocks(t *testing.T) {
	testRender(`
$$
a^2 + b^2 = c^2
$$
`,
		`<math display="block" displaystyle="true"><semantics><mrow><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></mrow><annotation encoding="application/x-tex">a^2 + b^2 = c^2 </annotation></semantics></math>`,
		t,
	)
	testRender("$$", "<p>$$</p>", t)
}

func TestEscapedChars(t *testing.T) {
	testRender(
		"This is \\**escaped bold**", "<p>This is **escaped bold**</p>", t)
}

func TestContainers(t *testing.T) {
	testRender(`
::: note A NOTE
This is some text in a note.
:::
`,
		`<div class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
		t,
	)
	testRender(`
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
`,
		`<div class="znak-container quote"><p class="quote-heading"><b><a href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
		t,
	)
	testRender(`
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
`,
		`<div class="znak-container quote bold"><p class="quote-heading"><b><a href="https://zerolimits.dev">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
		t,
	)
	testRender(`
::: warning
This is some text in a warning.
:::
`,
		`<div class="znak-container warning"><p class="warning-heading"><b>WARNING</b></p><p>This is some text in a warning.</p></div>`,
		t,
	)
	testRender(`
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
`,
		`<div class="znak-container block1"><p class="block1-heading"><b>This is the outer container</b></p><p>You can have some text here.</p><div class="znak-container block2"><p class="block2-heading"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>`,
		t,
	)
	testRenderOneOf(`
::: note A NOTE {id="my-note"}
This is some text in a note.
:::
`,
		[]string{
			`<div id="my-note" class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
			`<div class="znak-container note" id="my-note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
		},
		t,
	)
	testRender(`
::: note A NOTE
This is some text in a note.
:::

::: warning
This is some text in a warning.
:::
`,
		`<div class="znak-container note"><p class="note-heading"><b>A NOTE</b></p><p>This is some text in a note.</p></div><div class="znak-container warning"><p class="warning-heading"><b>WARNING</b></p><p>This is some text in a warning.</p></div>`,
		t,
	)
}

func TestMisc(t *testing.T) {
	testRender(`
This is a multi line
string
`, "<p>This is a multi linestring</p>", t)
	testRender(
		"> This is quite a **bold** statement!",
		"<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>",
		t,
	)
}

func TestEmptyBlocks(t *testing.T) {
	testRender(":::", "<p>:::</p>", t)
	testRender("```", "<p>```</p>", t)
	testRender("****", "<p>****</p>", t)
	testRender("__", "<p>__</p>", t)
	testRender("$$$$", "<p>$$$$</p>", t)
	testRender("[](https://zerolimits.dev)", "<p>[](https://zerolimits.dev)</p>", t)
	testRender("[link]()", "<p>[link]()</p>", t)
	testRender("^^", "<p>^^</p>", t)
	testRender("~~", "<p>~~</p>", t)
	testRender("====", "<p>====</p>", t)
	testRender("~~~~", "<p>~~~~</p>", t)
}

func TestFrontmatter(t *testing.T) {
	testRender(`
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
`,
		"<hr /><p>I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.</p>",
		t,
	)

}
