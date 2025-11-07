A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

You can add this library to your project with:

```bash
cargo add --git https://github.com/noClaps/znak highlight
```

## Languages

Highlight has a number of languages built in, but none are enabled by default to keep the bundle size small. You can enable the languages you'd like by enabling the respective features on the crate:

```toml
# Cargo.toml

[dependencies]
highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = [
  "bash",
  "c",
  "python",
  "typescript"
] }
```

If you'd like to enable all languages, you can do so by enabling the `all` feature:

```toml
# Cargo.toml

[dependencies]
highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = ["all"] }
```
