function get() {
    lang=$(basename $(dirname $1))
    output_dir="crates/highlight/queries/$lang/"
    mkdir -p $output_dir
    curl -LO $1 --output-dir $output_dir
}

# agda
get https://github.com/haohanyang/agda-zed/raw/refs/heads/master/languages/agda/highlights.scm
# bash
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/highlights.scm
# c
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/injections.scm
# cpp
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/injections.scm
# css
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/highlights.scm
# go
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/injections.scm
# haskell
get https://github.com/zed-extensions/haskell/raw/refs/heads/main/languages/haskell/highlights.scm
# html
get https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/injections.scm
# java
get https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/highlights.scm
get https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/injections.scm
get https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/locals.scm
# javascript
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/injections.scm
# jsdoc
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsdoc/highlights.scm
# json
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/json/highlights.scm
# ocaml
get https://github.com/zed-extensions/ocaml/raw/refs/heads/main/languages/ocaml/highlights.scm
# php
get https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/highlights.scm
get https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/injections.scm
# python
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/highlights.scm
# regex
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/regex/highlights.scm
# ruby
get https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/highlights.scm
get https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/injections.scm
# rust
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/injections.scm
# scala
get https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/highlights.scm
get https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/injections.scm
get https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/locals.scm
# typescript
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/highlights.scm
get https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/injections.scm
