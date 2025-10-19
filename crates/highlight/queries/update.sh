function get() {
    lang=$1
    output_dir="crates/highlight/queries/$lang/"
    mkdir -p $output_dir
    curl -LO $2 --output-dir $output_dir
}

# cpp
get cpp https://github.com/tree-sitter/tree-sitter-cpp/raw/refs/heads/master/queries/injections.scm
# javascript
get javascript https://github.com/tree-sitter/tree-sitter-javascript/raw/refs/heads/master/queries/highlights-params.scm
# julia
get julia https://github.com/tree-sitter/tree-sitter-julia/raw/refs/heads/master/queries/highlights.scm
get julia https://github.com/tree-sitter/tree-sitter-julia/raw/refs/heads/master/queries/locals.scm
# php
get php https://github.com/tree-sitter/tree-sitter-php/raw/refs/heads/master/queries/injections-text.scm
