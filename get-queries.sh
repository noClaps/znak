getquery() {
    local url="$1"
    local path=$(echo "$url" | sed 's|https\?://[^/]*||')
    local filename=$(basename "$path")
    local parent=$(dirname "$path")
    local last_dir=$(basename "$parent")
    curl -sL $1 -o "highlight/queries/$last_dir/$filename"
}

# Agda
getquery 'https://github.com/haohanyang/agda-zed/raw/refs/heads/master/languages/agda/highlights.scm'

# Bash
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/highlights.scm'

# C
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/injections.scm'

# C++
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/injections.scm'

# CSS
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/highlights.scm'

# Go
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/injections.scm'

# Haske
getquery 'https://github.com/zed-extensions/haskell/raw/refs/heads/main/languages/haskell/highlights.scm'

# HTML
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/injections.scm'

# Java
getquery 'https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/highlights.scm'
getquery 'https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/injections.scm'
getquery 'https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/locals.scm'

# JavaScript
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/injections.scm'

# JSDoc
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsdoc/highlights.scm'

# JSON
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/json/highlights.scm'

# OCaml
getquery 'https://github.com/zed-extensions/ocaml/raw/refs/heads/main/languages/ocaml/highlights.scm'

# PHP
getquery 'https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/highlights.scm'
getquery 'https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/injections.scm'

# Python
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/highlights.scm'

# Regex
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/regex/highlights.scm'

# Ruby
getquery 'https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/highlights.scm'
getquery 'https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/injections.scm'

# Rust
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/injections.scm'

# Scala
getquery 'https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/highlights.scm'
getquery 'https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/injections.scm'

# TSX
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/injections.scm'

# TypeScript
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/highlights.scm'
getquery 'https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/injections.scm'
