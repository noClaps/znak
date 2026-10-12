# Znak

A new markup language I'm working on.

## Build instructions

Clone the repository:

```sh
git clone https://codeberg.org/noClaps/znak.git
cd znak
```

Build the project:

```sh
cargo build --release
```

## Syntax

### Comments

Comments are lines (after skipping leading whitespace) that start with `\\ `, except inside code blocks and inline code.

```znak
\\ This is a comment
```

### Frontmatter

```znak
---
key = value
title = a title that contains = equal signs
---
```

converts to

```json
{
  "key": "value",
  "title": "a title that contains = equal signs"
}
```

### Headings

```znak
= heading 1

== heading 2

=== heading 3

==== heading 4

===== heading 5

====== heading 6
```

### Blockquotes

```znak
> blockquote content
>
> = heading in blockquote
>
>-- blockquote attribution
>-- over multiple lines
>-- combined into one
```

### Lists

```znak
- unordered list item
- list item
  - 2 space indentation
  - nested list item
    + nested ordered list item

+ ordered list item
+ list item
  + 2 space indentation
  + nested list item
    - nested unordered list item
```

### Code blocks

`````znak
```
Code here
```

````znak
Nested code blocks with additional backticks

```py
print("nested hello world")
```
````
`````

### Containers

```znak
::: name [Inline markup](https://example.org)
Normal block content here.

= Including headings
:::
```

```znak
:::: block1 A title
A container with 4 colons

::: block2
A nested container
:::
::::
```

### Block math

```znak
$$
a^2 + b^2 = c^2
$$
```

### Tables

```znak
| Table | header | here  |
| ----- | ------ | ----- |
| Table | body   | here  |
| left  | center | right |
| ----- | ------ | ----- |
| Table | footer | here  |
```

### Horizontal rule

Any line consisting of only hyphens `-` with at least 3 hyphens is a horiontal rule

```znak
---

-----
```

### Images

```znak
![Figure caption and alt text]<https://example.org/image/url.png>
```

### Inline formatting

```znak
This is a sentence with *bold*, _italics_, ^superscript^, `inline code`, [a link]<https://example.org>,
<https://example.org/a/bare/link>, and inline $x+y$ math.

You can escape these by escaping the first character with `\`:

This is a sentence with \*bold\*, \_italics\_, \^superscript\^, \`inline code\`, \[a link]\<https://example.org>,
\<https://example.org/a/bare/link>, and inline \$x+y$ math.
```

### Raw blocks

```znak
{{
<p>This contains raw code that'll be embedded as-is into the final output<p>
}}

It can be used to embed raw {{<html>}}.
```
