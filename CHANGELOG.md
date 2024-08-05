# @noclaps/znak

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
