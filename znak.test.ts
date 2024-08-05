import Znak from "./index.ts";
import katex from "katex";
import { expect } from "bun:test";
import { codeToHtml } from "shiki";

function test(md: string) {
  return new Znak(md).renderToHTML();
}

// Headings
expect(await test("# Heading 1")).toBe(`<h1 id="heading-1">Heading 1</h1>`);
expect(await test("## Heading 2")).toBe(`<h2 id="heading-2">Heading 2</h2>`);
expect(await test("### Heading 3")).toBe(`<h3 id="heading-3">Heading 3</h3>`);
expect(await test("#### Heading 4")).toBe(`<h4 id="heading-4">Heading 4</h4>`);
expect(await test("##### Heading 5")).toBe(`<h5 id="heading-5">Heading 5</h5>`);
expect(await test("###### Heading 6")).toBe(
  `<h6 id="heading-6">Heading 6</h6>`
);

// Horizontal rule
expect(await test("---")).toBe("<hr></hr>");
expect(await test("-----")).toBe("<hr></hr>");
expect(await test("-------asdsa")).not.toBe("<hr></hr>");

// Inline formatting
expect(await test("**bold text**")).toBe("<p><strong>bold text</strong></p>");
expect(await test("_italic text_")).toBe("<p><em>italic text</em></p>");
expect(await test("_**bold and italic text**_")).toBe(
  "<p><em><strong>bold and italic text</strong></em></p>"
);
expect(await test("**_bold and italic text_**")).toBe(
  "<p><strong><em>bold and italic text</em></strong></p>"
);
expect(await test("This is some `inline code`")).toBe(
  "<p>This is some <code>inline code</code></p>"
);
expect(await test("This is a [link](https://zerolimits.dev)")).toBe(
  `<p>This is a <a href="https://zerolimits.dev">link</a></p>`
);
expect(await test("This is a **[bold link](https://zerolimits.dev)**")).toBe(
  `<p>This is a <strong><a href="https://zerolimits.dev">bold link</a></strong></p>`
);
expect(await test("This is a [**bold link**](https://zerolimits.dev)")).toBe(
  `<p>This is a <a href="https://zerolimits.dev"><strong>bold link</strong></a></p>`
);
expect(await test("This is an _[italic link](https://zerolimits.dev)_")).toBe(
  `<p>This is an <em><a href="https://zerolimits.dev">italic link</a></em></p>`
);
expect(await test("This is an [_italic link_](https://zerolimits.dev)")).toBe(
  `<p>This is an <a href="https://zerolimits.dev"><em>italic link</em></a></p>`
);
expect(await test("This is a `[code link](https://zerolimits.dev)`")).toBe(
  `<p>This is a <code>[code link](https://zerolimits.dev)</code></p>`
);
expect(await test("This is a [`code link`](https://zerolimits.dev)")).toBe(
  `<p>This is a <a href="https://zerolimits.dev"><code>code link</code></a></p>`
);
expect(await test("This is formatting inside a `**code** _block_`")).toBe(
  "<p>This is formatting inside a <code>**code** _block_</code></p>"
);
expect(await test(`~~strikethrough~~`)).toBe("<p><s>strikethrough</s></p>");
expect(await test("==highlight==")).toBe("<p><mark>highlight</mark></p>");
expect(await test("~sub~script")).toBe("<p><sub>sub</sub>script</p>");
expect(await test("^super^script")).toBe("<p><sup>super</sup>script</p>");
expect(
  await test(
    "[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)"
  )
).toBe(
  `<p><a href="https://en.wikipedia.org/wiki/Rust_(programming_language)">link with parentheses</a></p>`
);
expect(await test("Inline $$x+y$$ math")).toBe(
  `<p>Inline ${katex.renderToString("x+y")} math</p>`
);

// Blockquotes
expect(await test(`> This is text in a blockquote`)).toBe(
  "<blockquote><p>This is text in a blockquote</p></blockquote>"
);
expect(
  await test(`
> This is a
>
> multiline blockquote
  `)
).toBe("<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>");

// Images
expect(await test(`![alt text](https://picsum.photos/300)`)).toBe(
  `<figure><img src="https://picsum.photos/300" alt="alt text"></img><figcaption>alt text</figcaption></figure>`
);

// Code block
expect(await test('```py\nprint("Your code here")\n```')).toBe(
  await codeToHtml(`print("Your code here")`, {
    lang: "py",
    theme: "github-dark",
  })
);

// Lists
expect(
  await test(`
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space or 1 tab indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want
`)
).toBe(
  "<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>"
);
expect(
  await test(`
1. list item 1
2. list item 2
3. list item 3
	1. nested list item 1 (3 space or 1 tab indentation allowed)
	2. nested list item 2
		1. You can nest as far as you want
`)
).toBe(
  "<ol><li><p>list item 1</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ol><li><p>nested list item 1 (3 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ol><li><p>You can nest as far as you want</p></li></ol></li></ol></li></ol>"
);
expect(
  await test(`
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space or 1 tab indentation allowed)
  - nested list item 2
    - You can nest as far as you want
`)
).toBe(
  "<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>"
);
expect(
  await test(`
- list item 1 (only - allowed for list)
- list item 2
- list item 3
	- nested list item 1 (2 space or 1 tab indentation allowed)
	- nested list item 2
		- You can nest as far as you want
`)
).toBe(
  "<ul><li><p>list item 1 (only - allowed for list)</p></li><li><p>list item 2</p></li><li><p>list item 3</p><ul><li><p>nested list item 1 (2 space or 1 tab indentation allowed)</p></li><li><p>nested list item 2</p><ul><li><p>You can nest as far as you want</p></li></ul></li></ul></li></ul>"
);

// Tables
expect(
  await test(`
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
`)
).toBe(
  `<table><thead><tr><th align="left">title</th><th align="center">description</th><th align="right">heading 1</th><th align="">heading 2</th></tr></thead><tbody><tr><td align="left">left-aligned</td><td align="center">center-aligned</td><td align="right">right-aligned</td><td align="">default text alignment</td></tr></tbody></table>`
);

// HTML Elements
expect(
  await test(`
<div>
Content here
</div>
`)
).toBe(`<div>
Content here
</div>
`);

// Math blocks
expect(
  await test(`
$$
a^2 + b^2 = c^2
$$
`)
).toBe(katex.renderToString("a^2 + b^2 = c^2", { displayMode: true }));

// Escaped characters
expect(await test("This is \\**escaped bold**")).toBe(
  "<p>This is *<em>escaped bold</em>*</p>"
);

// Containers
expect(
  await test(`
::: note A NOTE
This is some text in a note.
:::
  `)
).toBe(
  `<div class="note"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`
);
expect(
  await test(`
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
  `)
).toBe(
  `<div class="quote"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`
);
expect(
  await test(`
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
  `)
).toBe(
  `<div class="quote" class="bold"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`
);
expect(
  await test(`
::: warning
This is some text in a warning.
:::
  `)
).toBe(
  `<div class="warning"><p class=\"warning-heading\"><b>WARNING</b></p><p>This is some text in a warning.</p></div>`
);
expect(
  await test(`
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
`)
).toBe(
  `<div class="block1"><p class="block1-heading"><b>This is the outer container</b></p><p>You can have some text here.</p><div class="block2"><p class="block2-heading"><b>This is the inner container</b></p><p>This can have some more text.</p></div></div>`
);

// Misc
expect(
  await test(`
This is a multi line
string
`)
).toBe("<p>This is a multi linestring</p>");
expect(await test("> This is quite a **bold** statement!")).toBe(
  "<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>"
);
// Empty blocks
expect(await test(":::")).toBe("<p>:::</p>");
expect(await test("```")).toBe("<p>```</p>");
