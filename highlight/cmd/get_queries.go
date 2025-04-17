package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"
)

func main() {
	urls := []string{
		// Agda
		"https://github.com/haohanyang/agda-zed/raw/refs/heads/master/languages/agda/highlights.scm",

		// Bash
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/bash/highlights.scm",

		// C
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/c/injections.scm",

		// C++
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/cpp/injections.scm",

		// CSS
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/css/highlights.scm",

		// Go
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/go/injections.scm",

		// Haskell
		"https://github.com/zed-extensions/haskell/raw/refs/heads/main/languages/haskell/highlights.scm",

		// HTML
		"https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/extensions/html/languages/html/injections.scm",

		// Java
		"https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/highlights.scm",
		"https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/injections.scm",
		"https://github.com/zed-extensions/java/raw/refs/heads/main/languages/java/locals.scm",

		// JavaScript
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/javascript/injections.scm",

		// JSDoc
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/jsdoc/highlights.scm",

		// JSON
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/json/highlights.scm",

		// OCaml
		"https://github.com/zed-extensions/ocaml/raw/refs/heads/main/languages/ocaml/highlights.scm",

		// PHP
		"https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/highlights.scm",
		"https://github.com/zed-extensions/php/raw/refs/heads/main/languages/php/injections.scm",

		// Python
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/python/highlights.scm",

		// Regex
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/regex/highlights.scm",

		// Ruby
		"https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/highlights.scm",
		"https://github.com/zed-extensions/ruby/raw/refs/heads/main/languages/ruby/injections.scm",

		// Rust
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/rust/injections.scm",

		// Scala
		"https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/highlights.scm",
		"https://github.com/scalameta/metals-zed/raw/refs/heads/main/languages/scala/injections.scm",

		// TSX
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/tsx/injections.scm",

		// TypeScript
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/highlights.scm",
		"https://github.com/zed-industries/zed/raw/refs/heads/main/crates/languages/src/typescript/injections.scm",
	}

	done := make(chan bool)

	for _, url := range urls {
		go func(url string) {
			log.Println("Fetching", url)
			res, err := http.Get(url)
			if err != nil {
				log.Fatalln(err)
			}
			defer res.Body.Close()

			output, err := io.ReadAll(res.Body)
			if err != nil {
				log.Fatalln(err)
			}

			pathSegments := strings.Split(url, "/")
			filePath := strings.Join(pathSegments[len(pathSegments)-2:], "/")
			err = os.WriteFile(fmt.Sprintf("highlight/queries/%s", filePath), output, 0o666)
			if err != nil {
				log.Fatalln(err)
			}

			done <- true
		}(url)
	}

	for _ = range urls {
		<-done
	}
}
