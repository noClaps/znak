import { render, headings } from "./index.ts";
import Temml from "temml";
import { expect, test } from "bun:test";
import { codeToHtml } from "shiki";

async function testRender(md: string) {
  return await render(md);
}
function testHeadings(md: string) {
  return headings(md);
}

test("Headings", async () => {
  expect(await testRender("# Heading 1")).toBe(
    `<h1 id="heading-1">Heading 1</h1>`,
  );
  expect(await testRender("## Heading 2")).toBe(
    `<h2 id="heading-2">Heading 2</h2>`,
  );
  expect(await testRender("### Heading 3")).toBe(
    `<h3 id="heading-3">Heading 3</h3>`,
  );
  expect(await testRender("#### Heading 4")).toBe(
    `<h4 id="heading-4">Heading 4</h4>`,
  );
  expect(await testRender("##### Heading 5")).toBe(
    `<h5 id="heading-5">Heading 5</h5>`,
  );
  expect(await testRender("###### Heading 6")).toBe(
    `<h6 id="heading-6">Heading 6</h6>`,
  );
  expect(testHeadings("## Heading 2")).toEqual([
    { depth: 2, slug: "heading-2", title: "Heading 2" },
  ]);
  expect(testHeadings("### This_is-a🍪heading")).toEqual([
    { depth: 3, slug: "this-is-a--heading", title: "This_is-a🍪heading" },
  ]);
  expect(
    await testRender(`
## Heading
### Heading
#### Heading
`),
  ).toBe(
    `<h2 id="heading">Heading</h2><h3 id="heading-1">Heading</h3><h4 id="heading-2">Heading</h4>`,
  );
});

test("Horizontal rule", async () => {
  expect(await testRender("---")).toBe("<hr />");
  expect(await testRender("-----")).toBe("<hr />");
  expect(await testRender("-------asdsa")).not.toBe("<hr />");
});

test("Inline formatting", async () => {
  expect(await testRender("**bold text**")).toBe(
    "<p><strong>bold text</strong></p>",
  );
  expect(await testRender("_italic text_")).toBe("<p><em>italic text</em></p>");
  expect(await testRender("_**bold and italic text**_")).toBe(
    "<p><em><strong>bold and italic text</strong></em></p>",
  );
  expect(await testRender("**_bold and italic text_**")).toBe(
    "<p><strong><em>bold and italic text</em></strong></p>",
  );
  expect(await testRender("This is some `inline code`")).toBe(
    "<p>This is some <code>inline code</code></p>",
  );
  expect(await testRender("This is a [link](https://zerolimits.dev)")).toBe(
    `<p>This is a <a href="https://zerolimits.dev">link</a></p>`,
  );
  expect(
    await testRender("This is a **[bold link](https://zerolimits.dev)**"),
  ).toBe(
    `<p>This is a <strong><a href="https://zerolimits.dev">bold link</a></strong></p>`,
  );
  expect(
    await testRender("This is a [**bold link**](https://zerolimits.dev)"),
  ).toBe(
    `<p>This is a <a href="https://zerolimits.dev"><strong>bold link</strong></a></p>`,
  );
  expect(
    await testRender("This is an _[italic link](https://zerolimits.dev)_"),
  ).toBe(
    `<p>This is an <em><a href="https://zerolimits.dev">italic link</a></em></p>`,
  );
  expect(
    await testRender("This is an [_italic link_](https://zerolimits.dev)"),
  ).toBe(
    `<p>This is an <a href="https://zerolimits.dev"><em>italic link</em></a></p>`,
  );
  expect(
    await testRender("This is a `[code link](https://zerolimits.dev)`"),
  ).toBe(`<p>This is a <code>[code link](https://zerolimits.dev)</code></p>`);
  expect(
    await testRender("This is a [`code link`](https://zerolimits.dev)"),
  ).toBe(
    `<p>This is a <a href="https://zerolimits.dev"><code>code link</code></a></p>`,
  );
  expect(
    await testRender("This is formatting inside a `**code** _block_`"),
  ).toBe("<p>This is formatting inside a <code>**code** _block_</code></p>");
  expect(await testRender(`~~strikethrough~~`)).toBe(
    "<p><s>strikethrough</s></p>",
  );
  expect(
    await testRender(`This is a sentence with ~~strikethrough~~ in it`),
  ).toBe("<p>This is a sentence with <s>strikethrough</s> in it</p>");
  expect(await testRender("==highlight==")).toBe(
    "<p><mark>highlight</mark></p>",
  );
  expect(await testRender(`This is a sentence with ==highlight== in it`)).toBe(
    "<p>This is a sentence with <mark>highlight</mark> in it</p>",
  );
  expect(await testRender("~sub~script")).toBe("<p><sub>sub</sub>script</p>");
  expect(await testRender(`This is a sentence with ~sub~script in it`)).toBe(
    "<p>This is a sentence with <sub>sub</sub>script in it</p>",
  );
  expect(await testRender("^super^script")).toBe(
    "<p><sup>super</sup>script</p>",
  );
  expect(await testRender(`This is a sentence with ^super^script in it`)).toBe(
    "<p>This is a sentence with <sup>super</sup>script in it</p>",
  );
  expect(
    await testRender(
      "[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)",
    ),
  ).toBe(
    `<p><a href="https://en.wikipedia.org/wiki/Rust_(programming_language)">link with parentheses</a></p>`,
  );
  expect(
    await testRender(
      "[Some [square braces] inside a link](https://zerolimits.dev)",
    ),
  ).toBe(
    `<p><a href="https://zerolimits.dev">Some [square braces] inside a link</a></p>`,
  );

  expect(await testRender("Inline $$x+y$$ math")).toBe(
    `<p>Inline ${Temml.renderToString("x+y")} math</p>`,
  );
  expect(
    await testRender("This is a **line** with multiple **bold** words"),
  ).toBe(
    "<p>This is a <strong>line</strong> with multiple <strong>bold</strong> words</p>",
  );
});

test("Blockquotes", async () => {
  expect(await testRender(`> This is text in a blockquote`)).toBe(
    "<blockquote><p>This is text in a blockquote</p></blockquote>",
  );
  expect(
    await testRender(`
> This is a
>
> multiline blockquote
  `),
  ).toBe(
    "<blockquote><p>This is a</p><p>multiline blockquote</p></blockquote>",
  );
});

test("Images", async () => {
  expect(await testRender(`![alt text](https://picsum.photos/300)`)).toBe(
    `<figure><img src="https://picsum.photos/300" alt="alt text" /><figcaption>alt text</figcaption></figure>`,
  );
  expect(
    await testRender(
      `![This contains a [link](https://picsum.photos)](https://picsum.photos/300)`,
    ),
  ).toBe(
    `<figure><img src="https://picsum.photos/300" alt="This contains a [link](https://picsum.photos)" /><figcaption>This contains a <a href="https://picsum.photos">link</a></figcaption></figure>`,
  );
});

test("Code block", async () => {
  expect(await testRender('```py\nprint("Your code here")\n```')).toBe(
    await codeToHtml(`print("Your code here")`, {
      lang: "py",
      theme: "github-dark",
    }),
  );
  expect(await testRender("```\nThis is some text in a code block\n```")).toBe(
    await codeToHtml("This is some text in a code block", {
      lang: "plaintext",
      theme: "github-dark",
    }),
  );
  expect(
    await testRender(
      "```skajdlas\nThis is for a language that doesn't exist\n```",
    ),
  ).toBe(
    await codeToHtml("This is for a language that doesn't exist", {
      lang: "plaintext",
      theme: "github-dark",
    }),
  );
});

test("Lists", async () => {
  expect(
    await testRender(`
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
    await testRender(`
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
    await testRender(`
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
    await testRender(`
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

test("Tables", async () => {
  expect(
    await testRender(`
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
`),
  ).toBe(
    `<table><thead><tr><th align="left">title</th><th align="center">description</th><th align="right">heading 1</th><th align="">heading 2</th></tr></thead><tbody><tr><td align="left">left-aligned</td><td align="center">center-aligned</td><td align="right">right-aligned</td><td align="">default text alignment</td></tr></tbody></table>`,
  );
});

test("HTML Elements", async () => {
  expect(
    await testRender(`
<div>
Content here
</div>
`),
  ).toBe(`<div>
Content here
</div>`);
  expect(await testRender(`<u>This element isn't closed`)).toBe(
    `<u>This element isn't closed`,
  );
});

test("Math blocks", async () => {
  expect(
    await testRender(`
$$
a^2 + b^2 = c^2
$$
`),
  ).toBe(Temml.renderToString("a^2 + b^2 = c^2", { displayMode: true }));
  expect(await testRender("$$")).toBe("<p>$$</p>");
});

test("Escaped characters", async () => {
  expect(await testRender("This is \\**escaped bold**")).toBe(
    "<p>This is **escaped bold**</p>",
  );
});

test("Containers", async () => {
  expect(
    await testRender(`
::: note A NOTE
This is some text in a note.
:::
  `),
  ).toBe(
    `<div class="znak-container note"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
  );
  expect(
    await testRender(`
::: quote A QUOTE {href="https://zerolimits.dev"}
This is some text in a quote.
:::
  `),
  ).toBe(
    `<div class="znak-container quote"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
  );
  expect(
    await testRender(`
::: quote A QUOTE {href="https://zerolimits.dev" class="bold"}
This is some text in a quote.
:::
  `),
  ).toBe(
    `<div class="znak-container quote bold"><p class=\"quote-heading\"><b><a href="https://zerolimits.dev" target="_blank" rel="noopener noreferrer">A QUOTE</a></b></p><p>This is some text in a quote.</p></div>`,
  );
  expect(
    await testRender(`
::: warning
This is some text in a warning.
:::
  `),
  ).toBe(
    `<div class="znak-container warning"><p class=\"warning-heading\"><b>WARNING</b></p><p>This is some text in a warning.</p></div>`,
  );
  expect(
    await testRender(`
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
    await testRender(`
::: note A NOTE {id="my-note"}
This is some text in a note.
:::
  `),
  ).toBe(
    `<div class="znak-container note" id="my-note"><p class=\"note-heading\"><b>A NOTE</b></p><p>This is some text in a note.</p></div>`,
  );
});

test("Misc", async () => {
  expect(
    await testRender(`
This is a multi line
string
`),
  ).toBe("<p>This is a multi linestring</p>");
  expect(await testRender("> This is quite a **bold** statement!")).toBe(
    "<blockquote><p>This is quite a <strong>bold</strong> statement!</p></blockquote>",
  );
});

test("Empty blocks", async () => {
  expect(await testRender(":::")).toBe("<p>:::</p>");
  expect(await testRender("```")).toBe("<p>```</p>");
  expect(await testRender("****")).toBe("<p>****</p>");
  expect(await testRender("__")).toBe("<p>__</p>");
  expect(await testRender("$$$$")).toBe("<p>$$$$</p>");
  expect(await testRender("[](https://zerolimits.dev)")).toBe(
    "<p>[](https://zerolimits.dev)</p>",
  );
  expect(await testRender("[link]()")).toBe("<p>[link]()</p>");
  expect(await testRender("^^")).toBe("<p>^^</p>");
  expect(await testRender("~~")).toBe("<p>~~</p>");
  expect(await testRender("====")).toBe("<p>====</p>");
  expect(await testRender("~~~~")).toBe("<p>~~~~</p>");
});
