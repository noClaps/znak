const queries: Record<
  string,
  { highlights: string; injections?: string; locals?: string }
> = {
  angular: {
    highlights:
      "https://github.com/nathansbradshaw/zed-angular/raw/refs/heads/main/languages/angular/highlights.scm",
    injections:
      "https://github.com/nathansbradshaw/zed-angular/raw/refs/heads/main/languages/angular/injections.scm",
  },
  bash: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/injections.scm",
  },
  c: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/injections.scm",
  },
  comment: {
    highlights:
      "https://github.com/thedadams/zed-comment/raw/refs/heads/main/languages/comment/highlights.scm",
  },
  css: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/injections.scm",
  },
  csv: {
    highlights:
      "https://github.com/huacnlee/zed-csv/raw/refs/heads/main/languages/csv/highlights.scm",
  },
  gitattributes: {
    highlights:
      "https://github.com/zed-extensions/git_firefly/raw/refs/heads/main/languages/gitattributes/highlights.scm",
  },
  gitignore: {
    highlights:
      "https://github.com/zed-extensions/git_firefly/raw/refs/heads/main/languages/gitignore/highlights.scm",
  },
  go: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/injections.scm",
  },
  graphql: {
    highlights:
      "https://github.com/11bit/zed-extension-graphql/raw/refs/heads/main/languages/graphql/highlights.scm",
  },
  html: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/injections.scm",
  },
  javascript: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/injections.scm",
  },
  jsdoc: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsdoc/highlights.scm",
  },
  json: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/json/highlights.scm",
  },
  jsonc: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsonc/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsonc/injections.scm",
  },
  julia: {
    highlights:
      "https://github.com/JuliaEditorSupport/zed-julia/raw/refs/heads/main/languages/julia/highlights.scm",
    injections:
      "https://github.com/JuliaEditorSupport/zed-julia/raw/refs/heads/main/languages/julia/injections.scm",
  },
  latex: {
    highlights:
      "https://github.com/rzukic/zed-latex/raw/refs/heads/main/languages/latex/highlights.scm",
    injections:
      "https://github.com/rzukic/zed-latex/raw/refs/heads/main/languages/latex/injections.scm",
  },
  lua: {
    highlights:
      "https://github.com/zed-extensions/lua/raw/refs/heads/main/languages/lua/highlights.scm",
  },
  make: {
    highlights:
      "https://github.com/caius/zed-make/raw/refs/heads/main/languages/make/highlights.scm",
    injections:
      "https://github.com/caius/zed-make/raw/refs/heads/main/languages/make/injections.scm",
  },
  markdown: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/markdown/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/markdown/injections.scm",
  },
  "markdown-inline": {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/markdown-inline/highlights.scm",
  },
  python: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/injections.scm",
  },
  regex: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/regex/highlights.scm",
  },
  rust: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/injections.scm",
  },
  scheme: {
    highlights:
      "https://github.com/zed-extensions/scheme/raw/refs/heads/main/languages/scheme/highlights.scm",
    injections:
      "https://github.com/zed-extensions/scheme/raw/refs/heads/main/languages/scheme/injections.scm",
  },
  sql: {
    highlights:
      "https://github.com/zed-extensions/sql/raw/refs/heads/main/languages/sql/highlights.scm",
  },
  swift: {
    highlights:
      "https://github.com/zed-extensions/swift/raw/refs/heads/main/languages/swift/highlights.scm",
    injections:
      "https://github.com/zed-extensions/swift/raw/refs/heads/main/languages/swift/injections.scm",
    locals:
      "https://github.com/zed-extensions/swift/raw/refs/heads/main/languages/swift/locals.scm",
  },
  toml: {
    highlights:
      "https://github.com/zed-extensions/toml/raw/refs/heads/main/languages/toml/highlights.scm",
    injections:
      "https://github.com/zed-extensions/toml/raw/refs/heads/main/languages/toml/injections.scm",
  },
  tsx: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/injections.scm",
  },
  typescript: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/injections.scm",
  },
  xml: {
    highlights:
      "https://github.com/sweetppro/zed-xml/raw/refs/heads/main/languages/xml/highlights.scm",
  },
  yaml: {
    highlights:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/yaml/highlights.scm",
    injections:
      "https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/yaml/injections.scm",
  },
};

Promise.all(
  Object.entries(queries).map(
    async ([name, { highlights, injections, locals }]) => {
      // @ts-expect-error
      Bun.write(
        `crates/tree-sitter-languages/queries/${name}/highlights.scm`,
        await get(highlights),
      );
      if (injections) {
        // @ts-expect-error
        Bun.write(
          `crates/tree-sitter-languages/queries/${name}/injections.scm`,
          await get(injections),
        );
      }
      if (locals) {
        // @ts-expect-error
        Bun.write(
          `crates/tree-sitter-languages/queries/${name}/locals.scm`,
          await get(locals),
        );
      }
    },
  ),
);

async function get(url: string): Promise<string> {
  let res = await fetch(url);
  if (!res.ok) {
    return "";
  }
  return res.text();
}

export {};
