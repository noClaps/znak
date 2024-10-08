# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like [Shiki](https://shiki.style) syntax highlighting, [KaTeX](https://katex.org), and heading IDs built-in.

The full documentation for Znak, such as the types and syntax, are available [here](https://docs.zerolimits.dev/znak).

## Usage

Install the package as a dependency to your project:

```sh
# Use the command for your package manager
npx jsr add @noclaps/znak
yarn dlx jsr add @noclaps/znak
pnpm dlx jsr add @noclaps/znak
bunx jsr add @noclaps/znak
deno add @noclaps/znak
```

and then import it in your code:

```ts
import Znak from "@noclaps/znak";

const text = "# Hello World"; // Your text to be rendered.
const renderedHTML = new Znak(text).renderToHTML();
// <h1 id="hello-world">Hello World</h1>
const headings = new Znak(text).headings();
// [{depth: 1, slug: "hello-world", title: "Hello World"}]
```
