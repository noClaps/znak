# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, LaTeX, and heading IDs built-in.

You can read the syntax [here](./docs/syntax.md).

## Installation

You can install it using Homebrew on macOS/Linux:

```sh
brew install noclaps/tap/lsdeps
```

or you can build from source:

```sh
cargo install --git https://gitlab.com/noClaps/znak-lang
```

## Usage

```
Usage: znak [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the Znak file to build to HTML

Options:
  -t, --theme <THEME>  Path to theme TOML file, leave empty if you don't want syntax highlighting
      --headings       Whether or not the CLI should return headings
  -h, --help           Print help
  -V, --version        Print version
```

You can use it by running:

```sh
znak path/to/file.md -t path/to/theme.toml
```

and the output HTML will be printed to `stdout`. If you want to write to a file, you can use the `>` operator, for example:

```sh
znak README.md -t theme.toml > out.html
```
