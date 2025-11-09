const names = [
  // agda
  "agda/highlights",
  // bash
  "bash/highlights",
  "bash/injections",
  // c
  "c/highlights",
  "c/injections",
  "c/locals",
  // c++
  "cpp/highlights",
  "cpp/injections",
  // css
  "css/highlights",
  "css/injections",
  // go
  "go/highlights",
  "go/injections",
  "go/locals",
  // haskell
  "haskell/highlights",
  "haskell/injections",
  "haskell/locals",
  // html
  "html/highlights",
  "html/injections",
  // java
  "java/highlights",
  "java/injections",
  // javascript
  "javascript/highlights",
  "javascript/injections",
  "javascript/locals",
  "jsx/highlights",
  "jsx/injections",
  "jsx/locals",
  // jsdoc
  "jsdoc/highlights",
  "jsdoc/injections",
  // json
  "json/highlights",
  // julia
  "julia/highlights",
  "julia/injections",
  "julia/locals",
  // ocaml
  "ocaml/highlights",
  "ocaml/injections",
  "ocaml/locals",
  "ocaml-interface/highlights",
  "ocaml-interface/injections",
  // php
  "php/highlights",
  "php/injections",
  "php-only/highlights",
  "php-only/injections",
  // python
  "python/highlights",
  "python/injections",
  "python/locals",
  // regex
  "regex/highlights",
  // ruby
  "ruby/highlights",
  "ruby/injections",
  "ruby/locals",
  // rust
  "rust/highlights",
  "rust/injections",
  "rust/locals",
  // scala
  "scala/highlights",
  "scala/injections",
  "scala/locals",
  // typescript
  "typescript/highlights",
  "typescript/injections",
  "typescript/locals",
  "tsx/highlights",
  "tsx/injections",
  "tsx/locals",
];

Promise.all(
  names.map(async (name) => {
    // @ts-expect-error
    Bun.write(
      `crates/tree-sitter-languages/queries/${name}.scm`,
      await get(name),
    );
  }),
);

async function get(name: string): Promise<string> {
  const [_, file] = name.split("/", 2);
  let res = await fetch(
    `https://github.com/helix-editor/helix/raw/refs/heads/master/runtime/queries/${name}.scm`,
  );
  if (!res.ok) {
    return "";
  }
  let query = await res.text();

  const inheritsLine = query.match(/^; inherits: ([\w,]+)$/m);
  if (inheritsLine) {
    const [line, match] = inheritsLine;
    const inherits = (
      await Promise.all(match.split(",").map((lang) => get(`${lang}/${file}`)))
    ).join("\n");
    query = query.replace(line, inherits);
  }

  return query;
}

export {};
