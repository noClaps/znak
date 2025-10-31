---
title: Document title
description: Document description
---

Your content here

> This is text in a blockquote.

> This is a
>
> multiline blockquote

```py
print("Your code here")
```

::: quote
This is a quote container
:::

::: note Note title
This is a note container
:::

::: warning Warning title {id="an-id"}
This is a warning container
:::

::: quote A wise quote {href="https://zerolimits.dev"}
The title above becomes [a wise quote](https://zerolimits.dev).

Oh yeah, you can use **formatting** _inside_ here. Everything works.
:::

:::: block1 This is the outer container
You can have some text here.

::: block2 This is the inner container
This can have some more text.
:::
::::

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

---

<div>
Content here
</div>

![alt text](https://picsum.photos/300)

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

> This is quite a **bold** statement!

$$
a^2 + b^2 = c^2
$$

1. list item 1
2. list item 2
3. list item 3
   1. nested list item 1 (3 space indentation)
   2. nested list item 2
      1. You can nest as far as you want

| title        |  description   |     heading 1 | heading 2              |
| :----------- | :------------: | ------------: | ---------------------- |
| left-aligned | center-aligned | right-aligned | default text alignment |

- list item 1 (only - allowed for list)
- list item 2
- list item 3
  - nested list item 1 (2 space indentation)
  - nested list item 2
    - You can nest as far as you want
