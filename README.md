# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like [Shiki](https://shiki.style) syntax highlighting, [KaTeX](https://katex.org), and heading IDs built-in.

## Usage

1. Install the package as a dependency to your project.

   ```sh
   # Use the command for your package manager
   npx jsr add @noclaps/znak
   yarn dlx jsr add @noclaps/znak
   pnpm dlx jsr add @noclaps/znak
   bunx jsr add @noclaps/znak
   deno add @noclaps/znak
   ```

2. Import the package in your code, and use it.

   ```ts
   import Znak from "@noclaps/znak";

   const text = "# Hello World"; // Your text to be rendered.
   const renderedHTML = await new Znak(text).renderToHTML();
   // <h1 id="hello-world">Hello World</h1>
   const headings = new Znak(text).headings();
   // [{depth: 1, slug: "hello-world", title: "Hello World"}]
   ```

You will have to include the KaTeX CSS file somewhere in your page for it to render correctly. See [KaTeX browser usage](https://katex.org/docs/browser) for instructions.

You will also need to set background and text colors for the `<code>` element in your CSS, as these are not set by default. This can be done with:

```css
code {
  background-color: #24292e; /* Default background color for "github-dark" */
  color: #e1e4e8; /* Default text color for "github-dark" */
}
```

## Types

```ts
class Znak {
  constructor(input: string, codeTheme: BundledTheme = "github-dark");
  async renderToHTML(): Promise<string>;
  headings(): Heading[];
}
```

`input`: The input text to be converted to HTML. This can be from a Markdown file as long as the syntax is supported by Znak. See the [documentation](#syntax) for the supported syntax.

`codeTheme`: The theme for code blocks. This is set to "github-dark" by default, and can be set to any of the syntax highlighting themes included in [Shiki](https://shiki.style/themes).

## Syntax

Most of the syntax is similar to what you may expect from Markdown.

### Headings

```md
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
```

Outputs:

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

### Blockquotes

```md
> This is text in a blockquote.

> This is a
>
> multiline blockquote
```

Outputs:

> This is text in a blockquote.

> This is a
>
> multiline blockquote

### Horizontal rule

```md
---
```

Outputs:

---

### Images

```md
![alt text](https://picsum.photos/300)
```

Outputs:

<figure><img src="https://picsum.photos/300" alt="alt text"></img><figcaption>alt text</figcaption></figure>

### Code block

````md
```py
print("Your code here")
```
````

Outputs:

```py
print("Your code here")
```

### Ordered list

```md
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (only 3 space indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want
```

Outputs:

1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space or 1 tab indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want

### Unordered list

```md
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space or 1 tab indentation allowed)
  - nested list item 2
    - You can nest as far as you want
```

Outputs:

- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (only 2 space indentation allowed)
  - nested list item 2
    - You can nest as far as you want

### Tables

```md
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
```

Outputs:

| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |

### HTML elements

```md
<div>
Content here
</div>
```

This will be output exactly as-is, without any formatting applied to it.

### Math block

Both math blocks and inline math follow $\LaTeX$ syntax.

```md
$$
a^2 + b^2 = c^2
$$
```

Outputs:

$$
a^2 + b^2 = c^2
$$

### Inline formatting

```md
**bold text**

_italic text_

`inline code`

~~strikethrough~~

==highlight==

~sub~script

^super^script

[link text](https://zerolimits.dev)

[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)

Inline $$x+y$$ math
```

Outputs:

**bold text**

_italic text_

`inline code`

~~strikethrough~~

<mark>highlight</mark>

<sub>sub</sub>script

<sup>super</sup>script

[link text](https://zerolimits.dev)

[link with parentheses](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)

Inline $x+y$ math

These can also be used inside most other text blocks, such as inside blockquotes:

```md
> This is quite a **bold** statement!
```

### Containers

You can add custom containers like:

```md
::: quote
This is a quote container
:::
```

```html
<div class="quote">
  <p class="quote-heading"><b>QUOTE</b></p>
  <p>This is a quote container</p>
</div>
```

These can also have a custom title:

```md
::: note Note title
This is a note container
:::
```

```html
<div class="note">
  <p class="note-heading"><b>Note title</b></p>
  <p>This is a note container</p>
</div>
```

and additional attributes:

```md
::: warning Warning title {id="an-id"}
This is a warning container
:::
```

```html
<div class="warning" id="an-id">
  <p class="warning-heading"><b>Warning title</b></p>
  <p>This is a warning container</p>
</div>
```

If an `href` attribute is provided, the title automatically becomes a link:

```md
::: quote A wise quote {href="https://zerolimits.dev"}
The title above becomes [a wise quote](https://zerolimits.dev).

Oh yeah, you can use **formatting** _inside_ here. Everything works.
:::
```

```html
<div class="quote" id="an-id">
  <p class="quote-heading"><b><a href="https://zerolimits.dev">A wise quote</a></b></p>
  <p>The title above becomes <a href="https://zerolimits.dev">a wise quote</a></p>
  <p>Oh yeah, you can use <strong>formatting</strong> <em>inside</em> here. Everything works.</p>
</div>
```

You can also nest containers by adding more colons to the outside one.

```md
:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::
```

```html
<div class="block1">
  <p class="block1-heading"><b>This is the outer container</b></p>
  <p>You can have some text here.</p>
  <div class="block2">
    <p class="block2-heading"><b>This is the inner container</b></p>
    <p>This can have some more text.</p>
  </div>
</div>
```

These are not styled by default and should be styled by you.
