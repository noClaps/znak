import { render, headings } from "./index.ts";
import katex from "katex";
import { expect, test } from "bun:test";
import { codeToHtml } from "shiki";

function testRender(md: string) {
	return render(md);
}
function testHeadings(md: string) {
	return headings(md);
}

test("Headings", () => {
	expect(testRender("# Heading 1")).toBe(`<h1 id="heading-1">Heading 1</h1>`);
	expect(testRender("## Heading 2")).toBe(`<h2 id="heading-2">Heading 2</h2>`);
	expect(testRender("### Heading 3")).toBe(`<h3 id="heading-3">Heading 3</h3>`);
	expect(testRender("#### Heading 4")).toBe(
		`<h4 id="heading-4">Heading 4</h4>`,
	);
	expect(testRender("##### Heading 5")).toBe(
		`<h5 id="heading-5">Heading 5</h5>`,
	);
	expect(testRender("###### Heading 6")).toBe(
		`<h6 id="heading-6">Heading 6</h6>`,
	);
	expect(testHeadings("## Heading 2")).toEqual([
		{ depth: 2, slug: "heading-2", title: "Heading 2" },
	]);
	expect(testHeadings("### This_is-a🍪heading")).toEqual([
		{ depth: 3, slug: "this-is-a--heading", title: "This_is-a🍪heading" },
	]);
	expect(
		testRender(`
## Heading
### Heading
#### Heading
`),
	).toBe(
		`<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>`,
	);
});

test("Horizontal rule", () => {
	expect(testRender("---")).toBe("<hr />");
	expect(testRender("-----")).toBe("<hr />");
	expect(testRender("-------asdsa")).not.toBe("<hr />");
});

test("Inline formatting", () => {
	expect(testRender("**bold text**")).toBe("<p><strong>bold text</strong></p>");
	expect(testRender("_italic text_")).toBe("<p><em>italic text</em></p>");
	expect(testRender("_**bold and italic text**_")).toBe(
		"<p><em><strong>bold and italic text</strong></em></p>",
	);
	expect(testRender("**_bold and italic text_**")).toBe(
		"<p><strong><em>bold and italic text</em></strong></p>",
	);
	expect(testRender("This is some `inline code`")).toBe(
		"<p>This is some <code>inline code</code></p>",
	);
	expect(testRender("This is a [link](https://zerolimits.dev)")).toBe(
		`<p>This is a <a href="https://zerolimits.dev">link</a></p>`,
	);
	expect(testRender("This is a **[bold link](https://zerolimits.dev)**")).toBe(
		`<p>This is a <strong><a href="https://zerolimits.dev">bold link</a></strong></p>`,
	);
	expect(testRender("This is a [**bold link**](https://zerolimits.dev)")).toBe(
		`<p>This is a <a href="https://zerolimits.dev"><strong>bold link</strong></a></p>`,
	);
	expect(testRender("This is an _[italic link](https://zerolimits.dev)_")).toBe(
		`<p>This is an <em><a href="https://zerolimits.dev">italic link</a></em></p>`,
	);
	expect(testRender("This is an [_italic link_](https://zerolimits.dev)")).toBe(
		`<p>This is an <a href="https://zerolimits.dev"><em>italic link</em></a></p>`,
	);
	expect(testRender("This is a `[code link](https://zerolimits.dev)`")).toBe(
		`<p>This is a <code>[code link](https://zerolimits.dev)</code></p>`,
	);
	expect(testRender("This is a [`code link`](https://zerolimits.dev)")).toBe(
		`<p>This is a <a href="https://zerolimits.dev"><code>code link</code></a></p>`,
	);
	expect(testRender("This is formatting inside a `**code** _block_`")).toBe(
		"<p>This is formatting inside a <code>**code** _block_</code></p>",
	);
	expect(testRender(`~~strikethrough~~`)).toBe("<p><s>strikethrough</s></p>");
	expect(testRender(`This is a sentence with ~~strikethrough~~ in it`)).toBe(
		"<p>This is a sentence with <s>strikethrough</s> in it</p>",
	);
	expect(testRender("==highlight==")).toBe("<p><mark>highlight</mark></p>");
	expect(testRender(`This is a sentence with ==highlight== in it`)).toBe(
		"<p>This is a sentence with <mark>highlight</mark> in it</p>",
	);
	expect(testRender("~sub~script")).toBe("<p><sub>sub</sub>script</p>");
	expect(testRender(`This is a sentence with ~sub~script in it`)).toBe(
		"<p>This is a sentence with <sub>sub</sub>script in it</p>",
	);
	expect(testRender("^super^script")).toBe("<p><sup>super</sup>script</p>");
	expect(testRender(`This is a sentence with ^super^script in it`)).toBe(
		"<p>This is a sentence with <sup>super</sup>script in it</p>",
	);
	expect(
		testRender(
			"[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)",
		),
	).toBe(
		`<p><a href="https://en.wikipedia.org/wiki/Rust_(programming_language)">link with parentheses</a></p>`,
	);
	expect(
		testRender("[Some [square braces] inside a link](https://zerolimits.dev)"),
	).toBe(
		`<p><a href="https://zerolimits.dev">Some [square braces] inside a link</a></p>`,
	);

	expect(testRender("Inline $$x+y$$ math")).toBe(
		`<p>Inline ${katex.renderToString("x+y")} math</p>`,
	);
	expect(testRender("This is a **line** with multiple **bold** words")).toBe(
		"<p>This is a <strong>line</strong> with multiple <strong>bold</strong> words</p>",
	);
});

test("Blockquotes", () => {
	expect(testRender(`> This is text in a blockquote`)).toBe(
		"<blockquote><p>This is text in a blockquote</p></blockquote>",
	);
	expect(
		testRender(`
> This is a
>
> multiline blockquote
  `),
	).toBe(
		"<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>",
	);
});

test("Images", () => {
	expect(testRender(`![alt text](https://picsum.photos/300)`)).toBe(
		`<figure><img src="https://picsum.photos/300" alt="alt text" /><figcaption>alt text</figcaption></figure>`,
	);
	expect(
		testRender(
			`![This contains a [link](https://picsum.photos)](https://picsum.photos/300)`,
		),
	).toBe(
		`<figure><img src="https://picsum.photos/300" alt="This contains a [link](https://picsum.photos)" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>`,
	);
});

test("Code block", async () => {
	expect(testRender('```py\nprint("Your code here")\n```')).toBe(
		await codeToHtml(`print("Your code here")`, {
			lang: "py",
			theme: "github-dark",
		}),
	);
	expect(testRender("```\nThis is some text in a code block\n```")).toBe(
		await codeToHtml("This is some text in a code block", {
			lang: "plaintext",
			theme: "github-dark",
		}),
	);
	expect(
		testRender("```skajdlas\nThis is for a language that doesn't exist\n```"),
	).toBe(
		await codeToHtml("This is for a language that doesn't exist", {
			lang: "plaintext",
			theme: "github-dark",
		}),
	);
});

test("Lists", () => {
	expect(
		testRender(`
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space or 1 tab indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want
`),
	).toBe(
		"<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
	);
	expect(
		testRender(`
1. list item 1
2. list item 2
3. list item 3
	1. nested list item 1 (3 space or 1 tab indentation allowed)
	2. nested list item 2
		1. You can nest as far as you want
`),
	).toBe(
		"<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>",
	);
	expect(
		testRender(`
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space or 1 tab indentation allowed)
  - nested list item 2
    - You can nest as far as you want
`),
	).toBe(
		"<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
	);
	expect(
		testRender(`
- list item 1 (only - allowed for list)
- list item 2
- list item 3
	- nested list item 1 (2 space or 1 tab indentation allowed)
	- nested list item 2
		- You can nest as far as you want
`),
	).toBe(
		"<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>",
	);
});

test("Tables", () => {
	expect(
		testRender(`
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
`),
	).toBe(
		`<table><thead><tr><th align="left">title</th><th align="center">description</th><th align="right">heading 1</th><th align="">heading 2</th></tr></thead><tbody><tr><td align="left">left-aligned</td><td align="center">center-aligned</td><td align="right">right-aligned</td><td align="">default text alignment</td></tr></tbody></table>`,
	);
});

test("HTML Elements", () => {
	expect(
		testRender(`
<div>
Content here
</div>
`),
	).toBe(`<div>
Content here
</div>
`);
});

test("Math blocks", () => {
	expect(
		testRender(`
$$
a^2 + b^2 = c^2
$$
`),
	).toBe(katex.renderToString("a^2 + b^2 = c^2", { displayMode: true }));
	expect(testRender("$$")).toBe("<p>$$</p>");
});

test("Escaped characters", () => {
	expect(testRender("This is \\**escaped bold**")).toBe(
		"<p>This is **escaped bold**</p>",
	);
});

test("Containers", () => {
	expect(
		testRender(`
::: note A NOTE
This is some text in a note.
:::
  `),
	).toBe(
		`<div class="znak-container note"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
	);
	expect(
		testRender(`
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
  `),
	).toBe(
		`<div class="znak-container quote"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
	);
	expect(
		testRender(`
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
  `),
	).toBe(
		`<div class="znak-container quote bold"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
	);
	expect(
		testRender(`
::: warning
This is some text in a warning.
:::
  `),
	).toBe(
		`<div class="znak-container warning"><p class=\"warning-heading\"><b>WARNING</b></p><p>This is some text in a warning.</p></div>`,
	);
	expect(
		testRender(`
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
`),
	).toBe(
		`<div class="znak-container block1"><p class="block1-heading"><b>This is the outer container</b></p><p>You can have some text here.</p><div class="znak-container block2"><p class="block2-heading"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>`,
	);
	expect(
		testRender(`
::: note A NOTE {id="my-note"}
This is some text in a note.
:::
  `),
	).toBe(
		`<div class="znak-container note" id="my-note"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
	);
});

test("Misc", () => {
	expect(
		testRender(`
This is a multi line
string
`),
	).toBe("<p>This is a multi linestring</p>");
	expect(testRender("> This is quite a **bold** statement!")).toBe(
		"<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>",
	);
});

test("Empty blocks", () => {
	expect(testRender(":::")).toBe("<p>:::</p>");
	expect(testRender("```")).toBe("<p>```</p>");
	expect(testRender("****")).toBe("<p>****</p>");
	expect(testRender("__")).toBe("<p>__</p>");
	expect(testRender("$$$$")).toBe("<p>$$$$</p>");
	expect(testRender("[](https://zerolimits.dev)")).toBe(
		"<p>[](https://zerolimits.dev)</p>",
	);
	expect(testRender("[link]()")).toBe("<p>[link]()</p>");
	expect(testRender("^^")).toBe("<p>^^</p>");
	expect(testRender("~~")).toBe("<p>~~</p>");
	expect(testRender("====")).toBe("<p>====</p>");
	expect(testRender("~~~~")).toBe("<p>~~~~</p>");
});
