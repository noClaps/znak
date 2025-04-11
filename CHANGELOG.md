# Znak

## V0.16.1

- Check line length in tables
- Fix minifying and formatting math. This broke after updating to [Treeblood](https://github.com/wyatt915/treeblood) v0.1.7, but I fixed it and added minification. Hopefully it won't break again in future updates.

## v0.16.0

### Breaking changes

- Rewrite in Go

## v0.15.1

### Other changes

- Fix bug where frontmatter was leaking into content.

## v0.15.0

### Breaking changes

- Remove executable. Znak no longer has a binary target, and is instead only a library.

### Other changes

- Rewrite inline formatting to fix special characters bug. The bug involved special characters (like `à`) being used inline in text, but since Rust iterates over bytes instead `char`s, they weren't being handled properly. The rewrite changed all the inline formatting code to use `char`s instead, and the bug seems to be fixed!
- Fix underlined text syntax in docs. The docs previously said `**underlined**` and now say `__underlined__`

## v0.14.1

### New features

- Add support for parsing frontmatter. You can now add frontmatter to the top of your file like so:

  ```md
  ---
  title: Document title
  description: Document description
  ---
  ```

  It will be skipped with regular HTML generation. If you'd like to get the frontmatter out, you can use the `frontmatter()` function in the crate, or the `--frontmatter` flag in the CLI.

### Other changes
- Fix extra line of text occurring after containers.
- Add docs for frontmatter syntax.

## v0.14.0

### Breaking changes

- Turn off highlighting if no theme is passed in. Previously the CLI would use the GitHub Dark theme, now code blocks are not highlighted at all.

### New features

- Add new underline syntax feature. You can now use `__underlined text__` to get <u>underlined text</u>.

### Other changes

- Update dependencies
- Add installation, usage and syntax docs. These were moved here from the `docs.zerolimits.dev` site.
- Fix `parse_headings()` only returning one heading.
- Add docs for `--headings` and `--help` flags.
- Fix `brew install` command in README. It used to point to the wrong formula.

## v0.13.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.6...v0.13.0)

### 🚀 Enhancements

- ⚠️ Rewrite in Rust into a CLI and crate ([4e74017](https://github.com/noClaps/znak-lang/commit/4e74017))

### 🩹 Fixes

- Use bun.lock instead of bun.lockb in Dockerfile ([7b2f128](https://github.com/noClaps/znak-lang/commit/7b2f128))

### 📖 Documentation

- Update Highlight URL to GitHub ([c6420f8](https://github.com/noClaps/znak-lang/commit/c6420f8))

### 🏡 Chore

- Update dependencies and switch to text lockfile ([5fe41a6](https://github.com/noClaps/znak-lang/commit/5fe41a6))
- Update dependencies ([eaf2995](https://github.com/noClaps/znak-lang/commit/eaf2995))
- Update dependencies ([22203ed](https://github.com/noClaps/znak-lang/commit/22203ed))
- Remove demo ([3610dde](https://github.com/noClaps/znak-lang/commit/3610dde))
- Add version script ([04ce431](https://github.com/noClaps/znak-lang/commit/04ce431))

### 🤖 CI

- Remove copying bunfig.toml ([950d930](https://github.com/noClaps/znak-lang/commit/950d930))

#### ⚠️ Breaking Changes

- ⚠️ Rewrite in Rust into a CLI and crate ([4e74017](https://github.com/noClaps/znak-lang/commit/4e74017))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.12.6

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.5...v0.12.6)

### 📦 Build

- Use build.ts file and bundle types ([181cc62](https://github.com/noClaps/znak-lang/commit/181cc62))

### 🏡 Chore

- Fix repo url in package.json ([91940f3](https://github.com/noClaps/znak-lang/commit/91940f3))
- Remove bunfig.toml ([303c64e](https://github.com/noClaps/znak-lang/commit/303c64e))
- Update dependencies ([90e7481](https://github.com/noClaps/znak-lang/commit/90e7481))
- Update dependencies ([75c07cc](https://github.com/noClaps/znak-lang/commit/75c07cc))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.12.5

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.4...v0.12.5)

### 🏡 Chore

- Update dependencies ([3f1b58f](https://github.com/noClaps/znak-lang/commit/3f1b58f))
- Publish to npmjs.com instead ([cb48d1d](https://github.com/noClaps/znak-lang/commit/cb48d1d))
- Build output before publishing ([bc35402](https://github.com/noClaps/znak-lang/commit/bc35402))

### 🤖 CI

- Use npm config ([e7c5d3f](https://github.com/noClaps/znak-lang/commit/e7c5d3f))
- Use npm commands instead of bun ([b0e938a](https://github.com/noClaps/znak-lang/commit/b0e938a))
- Install dependencies before building ([b7dba85](https://github.com/noClaps/znak-lang/commit/b7dba85))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.12.4

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.3...v0.12.4)

### 📖 Documentation

- Fix type name in doc comment ([6e366db](https://github.com/noClaps/znak-lang/commit/6e366db))

### 🏡 Chore

- **demo:** Fix Dockerfile' ([d69dbfb](https://github.com/noClaps/znak-lang/commit/d69dbfb))
- Remove unused import ([64ae66e](https://github.com/noClaps/znak-lang/commit/64ae66e))
- Remove unused import ([03d0246](https://github.com/noClaps/znak-lang/commit/03d0246))
- Update dependencies ([e310f17](https://github.com/noClaps/znak-lang/commit/e310f17))

### 🤖 CI

- Fix deploy job ([f928b56](https://github.com/noClaps/znak-lang/commit/f928b56))
- Fix deploy job again ([5b7c4d2](https://github.com/noClaps/znak-lang/commit/5b7c4d2))
- Don't run deploy when publishing ([27b5bb1](https://github.com/noClaps/znak-lang/commit/27b5bb1))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.12.3

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.2...v0.12.3)

### 🚀 Enhancements

- Publish to self-hosted registry ([39446a8](https://github.com/noClaps/znak-lang/commit/39446a8))
- Update dependencies ([29b4d0d](https://github.com/noClaps/znak-lang/commit/29b4d0d))

### 🤖 CI

- Deploy demo site to Fly.io ([96178d4](https://github.com/noClaps/znak-lang/commit/96178d4))
- Update CI to deploy to self-hosted registry ([0990329](https://github.com/noClaps/znak-lang/commit/0990329))
- Use Bun whoami to check if publishing is possible with Bun ([1b953f9](https://github.com/noClaps/znak-lang/commit/1b953f9))
- Remove -b from bun test ([3de97c6](https://github.com/noClaps/znak-lang/commit/3de97c6))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.12.2

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.1...v0.12.2)

### 🚀 Enhancements

- **demo:** Switch to Hono for Cloudflare Workers ([29c0ff6](https://github.com/noClaps/znak-lang/commit/29c0ff6))

### 🏡 Chore

- Version @noclaps/highlight dependency ([84186c6](https://github.com/noClaps/znak-lang/commit/84186c6))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.12.1

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.12.0...v0.12.1)

### 🤖 CI

- Use Homebrew image ([0f3985b](https://github.com/noClaps/znak-lang/commit/0f3985b))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.12.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.11.2...v0.12.0)

### 🚀 Enhancements

- ⚠️ Switch to using @noclaps/highlight for syntax highlighting ([0103b92](https://github.com/noClaps/znak-lang/commit/0103b92))

### 📖 Documentation

- Remove KaTeX from README ([2b4ec49](https://github.com/noClaps/znak-lang/commit/2b4ec49))

### 🏡 Chore

- Update dependencies ([3d58e72](https://github.com/noClaps/znak-lang/commit/3d58e72))
- Move typescript into dev deps ([9015e51](https://github.com/noClaps/znak-lang/commit/9015e51))
- Update dependencies ([cc881b7](https://github.com/noClaps/znak-lang/commit/cc881b7))

#### ⚠️ Breaking Changes

- ⚠️ Switch to using @noclaps/highlight for syntax highlighting ([0103b92](https://github.com/noClaps/znak-lang/commit/0103b92))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.11.2

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.11.1...v0.11.2)

### 💅 Refactors

- Inline and simplify image parsing code ([d6213df](https://github.com/noClaps/znak-lang/commit/d6213df))
- Simplify a lot of inline formatting parsing ([0b1d336](https://github.com/noClaps/znak-lang/commit/0b1d336))
- Simplify container parsing ([ed2ec26](https://github.com/noClaps/znak-lang/commit/ed2ec26))
- Further simplify container parsing ([37c6bbf](https://github.com/noClaps/znak-lang/commit/37c6bbf))
- Use array destructuring and slices in table parsing ([2345684](https://github.com/noClaps/znak-lang/commit/2345684))

### 📖 Documentation

- Make input param for render and heading functions compulsory ([7529fc3](https://github.com/noClaps/znak-lang/commit/7529fc3))

### 🏡 Chore

- **bench:** Remove unneeded import ([4406f7d](https://github.com/noClaps/znak-lang/commit/4406f7d))
- Update dependencies ([cf02cef](https://github.com/noClaps/znak-lang/commit/cf02cef))
- Update dependencies ([f62cb91](https://github.com/noClaps/znak-lang/commit/f62cb91))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.11.1

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.11.0...v0.11.1)

### 🚀 Enhancements

- Switch KaTeX for Temml ([b3ea42f](https://github.com/noClaps/znak-lang/commit/b3ea42f))
- **demo:** Use build and dev scripts instead of Makefile ([fe3a198](https://github.com/noClaps/znak-lang/commit/fe3a198))

### 🔥 Performance

- Use Shiki's codeToHtml instead of making new highlighter ([d3daadf](https://github.com/noClaps/znak-lang/commit/d3daadf))

### 🩹 Fixes

- Fix errors when HTML element wasn't closed and trim HTML parsed output ([4372d02](https://github.com/noClaps/znak-lang/commit/4372d02))

### 💅 Refactors

- Inline blockquotes and headings functions ([8dc257d](https://github.com/noClaps/znak-lang/commit/8dc257d))
- Replace Records with Maps ([9d099c0](https://github.com/noClaps/znak-lang/commit/9d099c0))
- Inline code blocks parsing ([3807564](https://github.com/noClaps/znak-lang/commit/3807564))
- **demo:** Use non-null assertion for query selectors ([927b053](https://github.com/noClaps/znak-lang/commit/927b053))

### 📖 Documentation

- Add a short description to exported functions ([7e5af95](https://github.com/noClaps/znak-lang/commit/7e5af95))

### 🏡 Chore

- Remove unneeded @types/katex package from dev deps ([c06d6a8](https://github.com/noClaps/znak-lang/commit/c06d6a8))
- Update dependencies ([d57bc99](https://github.com/noClaps/znak-lang/commit/d57bc99))

### ✅ Tests

- Add benchmarking tests ([8c9d803](https://github.com/noClaps/znak-lang/commit/8c9d803))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.11.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.10.2...v0.11.0)

### 🚀 Enhancements

- Create new CodeTheme type and export it ([d313fee](https://github.com/noClaps/znak-lang/commit/d313fee))
- Export Heading type ([3ff7d2f](https://github.com/noClaps/znak-lang/commit/3ff7d2f))
- ⚠️ Don't bundle any syntax highlighting themes ([4cf7709](https://github.com/noClaps/znak-lang/commit/4cf7709))

### 🩹 Fixes

- Load all languages beforehand ([a5e59f9](https://github.com/noClaps/znak-lang/commit/a5e59f9))

### 📖 Documentation

- Update documentation to reflect new names and types ([34c1b75](https://github.com/noClaps/znak-lang/commit/34c1b75))

### 🏡 Chore

- Update dependencies ([2d03eec](https://github.com/noClaps/znak-lang/commit/2d03eec))

#### ⚠️ Breaking Changes

- ⚠️ Don't bundle any syntax highlighting themes ([4cf7709](https://github.com/noClaps/znak-lang/commit/4cf7709))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.10.2

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.10.1...v0.10.2)

### 🏡 Chore

- Add return type for render function ([3532e5c](https://github.com/noClaps/znak-lang/commit/3532e5c))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.10.1

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.10.0...v0.10.1)

### ✅ Tests

- Update tests to match new MathML output ([d3f3c30](https://github.com/noClaps/znak-lang/commit/d3f3c30))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.10.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.9.1...v0.10.0)

### 🚀 Enhancements

- Change to using codeToHtml for syntax highlighting ([7874520](https://github.com/noClaps/znak-lang/commit/7874520))
- Add demo site ([696a727](https://github.com/noClaps/znak-lang/commit/696a727))
- Create a faster function to parse headings, and use named exports ([fdea29b](https://github.com/noClaps/znak-lang/commit/fdea29b))
- ⚠️ Separate render and headings methods into their own functions ([28074e4](https://github.com/noClaps/znak-lang/commit/28074e4))
- ⚠️ Make render async ([d97de00](https://github.com/noClaps/znak-lang/commit/d97de00))
- ⚠️ Set KaTeX rendering mode to MathML ([54241a6](https://github.com/noClaps/znak-lang/commit/54241a6))

### 📖 Documentation

- Remove await from README example ([2ff0dba](https://github.com/noClaps/znak-lang/commit/2ff0dba))

### 🏡 Chore

- Change attrObject variable in container parser to const ([a494aed](https://github.com/noClaps/znak-lang/commit/a494aed))

### 🎨 Styles

- **demo:** Switch to using tabs for indentation ([1ad4822](https://github.com/noClaps/znak-lang/commit/1ad4822))

### 🤖 CI

- Don't publish to github Releases ([e6db68d](https://github.com/noClaps/znak-lang/commit/e6db68d))
- Remove github Releases publishing stages from `stages` list ([5a133e3](https://github.com/noClaps/znak-lang/commit/5a133e3))

#### ⚠️ Breaking Changes

- ⚠️ Separate render and headings methods into their own functions ([28074e4](https://github.com/noClaps/znak-lang/commit/28074e4))
- ⚠️ Make render async ([d97de00](https://github.com/noClaps/znak-lang/commit/d97de00))
- ⚠️ Set KaTeX rendering mode to MathML ([54241a6](https://github.com/noClaps/znak-lang/commit/54241a6))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.9.1

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.9.0...v0.9.1)

### 💅 Refactors

- Simplify syntax highlighter code ([a157637](https://github.com/noClaps/znak-lang/commit/a157637))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.9.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.8.3...v0.9.0)

### 🚀 Enhancements

- ⚠️ Match `Bun.escapeHTML()` for `escapeHTML` function ([6374e1e](https://github.com/noClaps/znak-lang/commit/6374e1e))
- ⚠️ Don't throw when an unsupported language is used ([17201e3](https://github.com/noClaps/znak-lang/commit/17201e3))

### 🩹 Fixes

- Fix invalid container openers not being parsed correctly ([87a9aac](https://github.com/noClaps/znak-lang/commit/87a9aac))
- Fix invalid code block openers not being parsed correctly ([43995d5](https://github.com/noClaps/znak-lang/commit/43995d5))

### 📖 Documentation

- Remove extra version from changelog ([8a9a1ea](https://github.com/noClaps/znak-lang/commit/8a9a1ea))

### 🏡 Chore

- Update dependencies ([77d1f51](https://github.com/noClaps/znak-lang/commit/77d1f51))
- Update syntax highlighting engine creation code ([5c054ce](https://github.com/noClaps/znak-lang/commit/5c054ce))
- Remove unneeded code ([2e09da3](https://github.com/noClaps/znak-lang/commit/2e09da3))

### ✅ Tests

- Add missing tests for syntax and use proper testing features ([689b442](https://github.com/noClaps/znak-lang/commit/689b442))

### 🎨 Styles

- Switch to using tabs for indentation ([6a69923](https://github.com/noClaps/znak-lang/commit/6a69923))

#### ⚠️ Breaking Changes

- ⚠️ Match `Bun.escapeHTML()` for `escapeHTML` function ([6374e1e](https://github.com/noClaps/znak-lang/commit/6374e1e))
- ⚠️ Don't throw when an unsupported language is used ([17201e3](https://github.com/noClaps/znak-lang/commit/17201e3))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.3

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.8.2...v0.8.3)

### 🩹 Fixes

- Escape HTML in code blocks when appropriate ([8a64b2d](https://github.com/noClaps/znak-lang/commit/8a64b2d))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.2

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.8.1...v0.8.2)

### 🩹 Fixes

- Initialise slugger occurrences properly ([3e28e25](https://github.com/noClaps/znak-lang/commit/3e28e25))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.1

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.8.0...v0.8.1)

### 🩹 Fixes

- Check whether a language is provided in code block ([5b83415](https://github.com/noClaps/znak-lang/commit/5b83415))

### 📖 Documentation

- Update usage example to be synchronous ([6192188](https://github.com/noClaps/znak-lang/commit/6192188))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.8.0

[compare changes](https://github.com/noClaps/znak-lang/compare/v0.7.3...v0.8.0)

### 🩹 Fixes

- Fix bug with slugger and add tests ([14b237c](https://github.com/noClaps/znak-lang/commit/14b237c))

### 💅 Refactors

- Use hast-like structure to create syntax tree ([a7e1387](https://github.com/noClaps/znak-lang/commit/a7e1387))
- Inline container generation into the parser ([105bbcc](https://github.com/noClaps/znak-lang/commit/105bbcc))
- Do math rendering during parsing and add it as a HastText node ([525242d](https://github.com/noClaps/znak-lang/commit/525242d))
- Use Shiki's codeToHast function for code blocks ([b8443a3](https://github.com/noClaps/znak-lang/commit/b8443a3))
- Replace github-slugger with self-made slugger ([c159223](https://github.com/noClaps/znak-lang/commit/c159223))
- ⚠️ Make parser synchronous ([bfa4818](https://github.com/noClaps/znak-lang/commit/bfa4818))

### 🤖 CI

- Don't publish to JSR using pipelines ([85bd19f](https://github.com/noClaps/znak-lang/commit/85bd19f))
- Don't publish to JSR using pipelines" ([c2dbd15](https://github.com/noClaps/znak-lang/commit/c2dbd15))

#### ⚠️ Breaking Changes

- ⚠️ Make parser synchronous ([bfa4818](https://github.com/noClaps/znak-lang/commit/bfa4818))

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
