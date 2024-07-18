# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, KaTeX, and heading IDs built-in.

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

![alt text](https://picsum.photos/300)

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
   1. nested list item 1 (only 3 space indentation allowed)
   2. nested list item 2
      1. You can nest as far as you want

### Unordered list

```md
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (only 2 space indentation allowed)
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
| title | description | heading 1 | heading 2 |
| :--- | :---: | ---: | --- |
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

Inline $x+y$ math
```

Outputs:

**bold text**

_italic text_

`inline code`

~~strikethrough~~

==highlight==

<sub>sub</sub>script

<sup>super</sup>script

[link text](https://zerolimits.dev)

Inline $x+y$ math

These can also be used inside most other text blocks, such as inside blockquotes:

```md
> This is quite a **bold** statement!
```
