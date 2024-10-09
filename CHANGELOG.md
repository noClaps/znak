# @noclaps/znak

## v0.10.0

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.9.1...v0.10.0)

### 🚀 Enhancements

- Change to using codeToHtml for syntax highlighting ([7874520](https://gitlab.com/noClaps/znak-lang/commit/7874520))
- Add demo site ([696a727](https://gitlab.com/noClaps/znak-lang/commit/696a727))
- Create a faster function to parse headings, and use named exports ([fdea29b](https://gitlab.com/noClaps/znak-lang/commit/fdea29b))
- ⚠️  Separate render and headings methods into their own functions ([28074e4](https://gitlab.com/noClaps/znak-lang/commit/28074e4))
- ⚠️  Make render async ([d97de00](https://gitlab.com/noClaps/znak-lang/commit/d97de00))
- ⚠️  Set KaTeX rendering mode to MathML ([54241a6](https://gitlab.com/noClaps/znak-lang/commit/54241a6))

### 📖 Documentation

- Remove await from README example ([2ff0dba](https://gitlab.com/noClaps/znak-lang/commit/2ff0dba))

### 🏡 Chore

- Change attrObject variable in container parser to const ([a494aed](https://gitlab.com/noClaps/znak-lang/commit/a494aed))

### 🎨 Styles

- **demo:** Switch to using tabs for indentation ([1ad4822](https://gitlab.com/noClaps/znak-lang/commit/1ad4822))

### 🤖 CI

- Don't publish to GitLab Releases ([e6db68d](https://gitlab.com/noClaps/znak-lang/commit/e6db68d))
- Remove GitLab Releases publishing stages from `stages` list ([5a133e3](https://gitlab.com/noClaps/znak-lang/commit/5a133e3))

#### ⚠️ Breaking Changes

- ⚠️  Separate render and headings methods into their own functions ([28074e4](https://gitlab.com/noClaps/znak-lang/commit/28074e4))
- ⚠️  Make render async ([d97de00](https://gitlab.com/noClaps/znak-lang/commit/d97de00))
- ⚠️  Set KaTeX rendering mode to MathML ([54241a6](https://gitlab.com/noClaps/znak-lang/commit/54241a6))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.9.1

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.9.0...v0.9.1)

### 💅 Refactors

- Simplify syntax highlighter code ([a157637](https://gitlab.com/noClaps/znak-lang/commit/a157637))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.9.0

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.8.3...v0.9.0)

### 🚀 Enhancements

- ⚠️  Match `Bun.escapeHTML()` for `escapeHTML` function ([6374e1e](https://gitlab.com/noClaps/znak-lang/commit/6374e1e))
- ⚠️  Don't throw when an unsupported language is used ([17201e3](https://gitlab.com/noClaps/znak-lang/commit/17201e3))

### 🩹 Fixes

- Fix invalid container openers not being parsed correctly ([87a9aac](https://gitlab.com/noClaps/znak-lang/commit/87a9aac))
- Fix invalid code block openers not being parsed correctly ([43995d5](https://gitlab.com/noClaps/znak-lang/commit/43995d5))

### 📖 Documentation

- Remove extra version from changelog ([8a9a1ea](https://gitlab.com/noClaps/znak-lang/commit/8a9a1ea))

### 🏡 Chore

- Update dependencies ([77d1f51](https://gitlab.com/noClaps/znak-lang/commit/77d1f51))
- Update syntax highlighting engine creation code ([5c054ce](https://gitlab.com/noClaps/znak-lang/commit/5c054ce))
- Remove unneeded code ([2e09da3](https://gitlab.com/noClaps/znak-lang/commit/2e09da3))

### ✅ Tests

- Add missing tests for syntax and use proper testing features ([689b442](https://gitlab.com/noClaps/znak-lang/commit/689b442))

### 🎨 Styles

- Switch to using tabs for indentation ([6a69923](https://gitlab.com/noClaps/znak-lang/commit/6a69923))

#### ⚠️ Breaking Changes

- ⚠️  Match `Bun.escapeHTML()` for `escapeHTML` function ([6374e1e](https://gitlab.com/noClaps/znak-lang/commit/6374e1e))
- ⚠️  Don't throw when an unsupported language is used ([17201e3](https://gitlab.com/noClaps/znak-lang/commit/17201e3))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.3

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.8.2...v0.8.3)

### 🩹 Fixes

- Escape HTML in code blocks when appropriate ([8a64b2d](https://gitlab.com/noClaps/znak-lang/commit/8a64b2d))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.2

[compare changes](https://gitlab.com/noClaps/znak-lang/compare/v0.8.1...v0.8.2)

### 🩹 Fixes

- Initialise slugger occurrences properly ([3e28e25](https://gitlab.com/noClaps/znak-lang/commit/3e28e25))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

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
- ⚠️ Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

### 🤖 CI

- Don't publish to JSR using pipelines ([85bd19f](https://gitlab.com/noClaps/znak-lang/commit/85bd19f))
- Don't publish to JSR using pipelines" ([c2dbd15](https://gitlab.com/noClaps/znak-lang/commit/c2dbd15))

#### ⚠️ Breaking Changes

- ⚠️ Make parser synchronous ([bfa4818](https://gitlab.com/noClaps/znak-lang/commit/bfa4818))

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
