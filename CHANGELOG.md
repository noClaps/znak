# @noclaps/znak

## v0.8.1

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.8.0...v0.8.1)

### 🩹 Fixes

- Check whether a language is provided in code block ([5b83415](https://gitlab.com/noClaps/znak-lang/commit/5b83415))

### 📖 Documentation

- Update usage example to be synchronous ([6192188](https://gitlab.com/noClaps/znak-lang/commit/6192188))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.0

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.7.3...v0.8.0)

### 🩹 Fixes

- Fix bug with slugger and add tests ([14b237c](https://gitlab.com/noClaps/znak-lang/commit/14b237c))

### 💅 Refactors

- Use hast-like structure to create syntax tree ([a7e1387](https://gitlab.com/noClaps/znak-lang/commit/a7e1387))
- Inline container generation into the parser ([105bbcc](https://gitlab.com/noClaps/znak-lang/commit/105bbcc))
- Do math rendering during parsing and add it as a HastText node ([525242d](https://gitlab.com/noClaps/znak-lang/commit/525242d))
- Use Shiki's codeToHast function for code blocks ([b8443a3](https://gitlab.com/noClaps/znak-lang/commit/b8443a3))
- Replace github-slugger with self-made slugger ([c159223](https://gitlab.com/noClaps/znak-lang/commit/c159223))
- ⚠️  Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

### 🤖 CI

- Don't publish to JSR using pipelines ([85bd19f](https://gitlab.com/noClaps/znak-lang/commit/85bd19f))
- Don't publish to JSR using pipelines" ([c2dbd15](https://gitlab.com/noClaps/znak-lang/commit/c2dbd15))

#### ⚠️ Breaking Changes

- ⚠️  Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.0

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.7.3...v0.1.0)

### 🩹 Fixes

- Fix bug with slugger and add tests ([14b237c](https://gitlab.com/noClaps/znak-lang/commit/14b237c))

### 💅 Refactors

- Use hast-like structure to create syntax tree ([a7e1387](https://gitlab.com/noClaps/znak-lang/commit/a7e1387))
- Inline container generation into the parser ([105bbcc](https://gitlab.com/noClaps/znak-lang/commit/105bbcc))
- Do math rendering during parsing and add it as a HastText node ([525242d](https://gitlab.com/noClaps/znak-lang/commit/525242d))
- Use Shiki's codeToHast function for code blocks ([b8443a3](https://gitlab.com/noClaps/znak-lang/commit/b8443a3))
- Replace github-slugger with self-made slugger ([c159223](https://gitlab.com/noClaps/znak-lang/commit/c159223))
- ⚠️  Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

### 🤖 CI

- Don't publish to JSR using pipelines ([85bd19f](https://gitlab.com/noClaps/znak-lang/commit/85bd19f))
- Don't publish to JSR using pipelines" ([c2dbd15](https://gitlab.com/noClaps/znak-lang/commit/c2dbd15))

#### ⚠️ Breaking Changes

- ⚠️  Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## 0.7.2

### Patch Changes

- 062c744: Add `znak-container` class to all custom containers

## 0.7.1

### Patch Changes

- c6c2016: Fix crashing on opening math block. This was caused by improper checks for whether there was a closing tag for the math block, both inline and blocks.

## 0.7.0

### Minor Changes

- df3649c: Remove `__` for bold and `*` for italics

### Patch Changes

- a5e44e0: Fix empty inline formatting blocks not rendering
- d3031cf: Fix crashes for empty code and container blocks

## 0.6.1

### Patch Changes

- 085eee5: Fix triple delimiter formatting for `***bold and italic***` and `___bold and italic___`

## 0.6.0

### Minor Changes

- 42ab604: Set code block theme colors by default. Previously this needed to be done by the user.

## 0.5.1

### Patch Changes

- 9488b7e: Fix imports

## 0.5.0

### Minor Changes

- 077bb71: Add support for both underscores (\_) and asterisks (\*) for bold and italic
- f7cb857: Add containers
- 5223327: Add support for escape characters
- 2ae9a6e: Add support for tabs in lists
- 9a187bf: Add support for horizontal rules longer than 3 dashes (-)

### Patch Changes

- 8f2e0bd: Properly trim blockquote lines
- 5889718: Fix issue with space after element tagname
- 4457e7b: Add tests

## 0.4.3

### Patch Changes

- b44735d: Fix brackets in link text
- d142e02: Fix code blocks closing on single backtick
- cedc2e3: Fix links with parentheses inside them
- 5d4c1b7: Fix code blocks
- eaf1224: Fix bug with HTML elements parsing
- 07fd878: Fix code blocks parsing
- 8c984fb: Fix HTML elements not working (again)
- 70aedab: Fix parsing condition for lists

## 0.4.2

### Patch Changes

- b94825f: Fix regex security issues
- 8300f6d: Remove CHANGELOG.md from JSR output

## 0.4.1

### Patch Changes

- 9931b07: Fix custom elements nested inside lists
- b40cde4: Add back check for line existence

## 0.4.0

### Minor Changes

- 1c8556d: Add `headings()` method to return the list of headings in the input file

## 0.3.0

### Minor Changes

- 8035bb6: Change inline math symbol from `# @noclaps/znak to `$` since a single dollar sign was causing issues when the intention was to display an actual money amount

## 0.2.0

### Minor Changes

- dabbca3: Add support for nested code blocks with increasing number of backticks outside

### Patch Changes

- 57168df: Add documentation to README.md

## 0.1.4

### Patch Changes

- b6419a1: Merge workflows into one

## 0.1.3

### Patch Changes

- 88c1821: Set up changesets for the repo

## 0.1.2

### Patch Changes

- 165b0f3: Fix bug with duplicate headings
