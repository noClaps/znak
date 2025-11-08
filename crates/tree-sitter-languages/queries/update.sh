function get() {
    lang=$1
    output_dir="crates/tree-sitter-languages/queries/$lang/"
    mkdir -p $output_dir
    curl -fsSLO $2 --output-dir $output_dir &
}

# agda
get agda https://github.com/haohanyang/agda-zed/raw/refs/heads/master/languages/agda/highlights.scm
# bash
get bash https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/highlights.scm
get bash https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/injections.scm
# c
get c https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/highlights.scm
get c https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/injections.scm
# cpp
get cpp https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/highlights.scm
get cpp https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/injections.scm
# css
get css https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/highlights.scm
get css https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/injections.scm
# go
get go https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/highlights.scm
get go https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/injections.scm
# haskell
get haskell https://github.com/zed-extensions/haskell/raw/refs/heads/main/languages/haskell/highlights.scm
get haskell https://github.com/zed-extensions/haskell/raw/refs/heads/main/languages/haskell/injections.scm
# html
get html https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/highlights.scm
get html https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/injections.scm
# java
get java https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/highlights.scm
get java https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/injections.scm
get java https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/locals.scm
# javascript
get javascript https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/highlights.scm
get javascript https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/injections.scm
# jsdoc
get jsdoc https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsdoc/highlights.scm
# json
get json https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/json/highlights.scm
# julia
get julia https://github.com/JuliaEditorSupport/zed-julia/raw/refs/heads/main/languages/julia/highlights.scm
get julia https://github.com/JuliaEditorSupport/zed-julia/raw/refs/heads/main/languages/julia/injections.scm
# ocaml
get ocaml https://github.com/zed-extensions/ocaml/raw/refs/heads/main/languages/ocaml/highlights.scm
# php
get php https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/highlights.scm
get php https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/injections.scm
# python
get python https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/highlights.scm
get python https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/injections.scm
# regex
get regex https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/regex/highlights.scm
# ruby
get ruby https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/highlights.scm
get ruby https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/injections.scm
# rust
get rust https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/highlights.scm
get rust https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/injections.scm
# scala
get scala https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/highlights.scm
get scala https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/injections.scm
get scala https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/locals.scm
# tsx
get tsx https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/highlights.scm
get tsx https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/injections.scm
# typescript
get typescript https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/highlights.scm
get typescript https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/injections.scm

wait
