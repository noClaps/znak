A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, LaTeX, and heading IDs built-in.

You can read the syntax below. You can also read the documentation for Highlight (the syntax highlighter in Znak) [here](highlight).

You can add this library to your project with:

```bash
cargo add --git https://github.com/noClaps/znak znak
```

# Syntax

Most of the syntax is similar to what you may expect from Markdown.

## Blockquotes

```md
> This is text in a blockquote.

> This is a
>
> multiline blockquote
```

## Code block

Code blocks are highlighted using [Highlight](highlight). To get syntax highlighting, you'll need to create a CSS file with your theme. See [Theme] for details on how to do this.

````md
```py
print("Your code here")
```
````

The language can be omitted if the code block should be rendered as plaintext.

## Containers

You can add custom containers like:

```md
::: quote
This is a quote container
:::
```

The default title of containers is simply the type of the container in full caps. For the Quote container above, its title would be "QUOTE", for example. However, these can also have a custom title:

```md
::: note Note title
This is a note container
:::
```

and additional attributes:

```md
::: warning Warning title {id="an-id"}
This is a warning container
:::
```

Attributes should be separated by spaces. These attributes are added to the container's outer `<div>` element. However, if an `href` attribute is provided, the title automatically becomes a link:

```md
::: quote A wise quote {href="https://zerolimits.dev"}
The title above becomes [a wise quote](https://zerolimits.dev).

Oh yeah, you can use **formatting** _inside_ here. Everything works.
:::
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

These are not styled by default and should be styled by you.

## Frontmatter

You can have frontmatter at the top of files, like so:

```md
---
title: Document title
description: Document description
---

Your content here
```

This will be skipped during regular parsing. If you'd like to get this frontmatter out, you can use [parse_frontmatter].

## Headings

```md
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
```

## Horizontal rule

```md
---
```

Any number of dashes can be used, as long as it is more than 3, and there are no other characters on the line. So `----` is also valid, but `-----nope` is not.

## HTML elements

```md
<div>
Content here
</div>
```

HTML elements are output as-is.

## Images

```md
![alt text](https://picsum.photos/300)
```

Images are output in a `<figure>` element, with the `alt` text also being a `<figcaption>`.

## Inline formatting

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

__underlined text__
```

These can also be used inside most other text blocks, such as inside blockquotes:

```md
> This is quite a **bold** statement!
```

## Math blocks

Both math blocks and inline math follow LaTeX syntax.

```md
$$
a^2 + b^2 = c^2
$$
```

## Ordered list

```md
1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space indentation)
   2. nested list item 2
      1. You can nest as far as you want
```

## Tables

```md
| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |
```

## Unordered list

```md
- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space indentation)
  - nested list item 2
    - You can nest as far as you want
```
