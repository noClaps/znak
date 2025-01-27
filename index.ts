import type { Theme } from "@noclaps/highlight";
import { parse, parseHeadings } from "./src/parser/index.ts";
import { renderer } from "./src/renderer.ts";
import type { Heading } from "./src/utils/slugger.ts";
import { githubTheme } from "./src/utils/syntax-highlighting.ts";

/**
 * A code theme. This is taken from `@noclaps/highlight`'s Theme type. You can
 * use it to make your own themes and pass them in to the `render()` function.
 *
 * @example
 * ```ts
 * import { type CodeTheme, render } from "@noclaps/znak";
 * const myTheme: CodeTheme = {
 *   // ...
 * };
 *
 * render(input, myTheme);
 * ```
 */
export type CodeTheme = Theme;
export { type Heading };

/**
 * A function that renders the input text to HTML.
 *
 * @param input The input text to be converted to HTML. This can be from a
 * Markdown file as long as the syntax is supported by Znak. See the
 * [documentation](https://github.com/noClaps/znak#syntax) for the supported
 * syntax.
 *
 * @param [codeTheme] The theme for code blocks. This is set to GitHub Dark
 * by default. To use a different theme, you must find one online, or create
 * one yourself:
 * ```ts
 * import { type CodeTheme } from "@noclaps/znak";
 * const theme: CodeTheme = {
 *   // ...
 * }
 *
 * render(input, theme)
 * ```
 * An example of a theme can be found in the [`@noclaps/highlight`
 * repository](https://github.com/noClaps/highlight/blob/main/highlight.test.ts)
 *
 * @returns An HTML string created from the input text.
 */
export function render(
  input: string,
  codeTheme: CodeTheme = githubTheme,
): string {
  const parserOutput = parse(input, codeTheme);
  return parserOutput.map((po) => renderer(po)).join("");
}

/**
 * A function that returns the headings in the given input text.
 *
 * @param input The input text to extract the headings from. This can be from
 * a Markdown file as long as the syntax is supported by Znak. See the
 * [documentation](https://github.com/noClaps/znak#syntax) for the supported
 * syntax.
 *
 * @returns A list of headings in the given input text.
 */
export function headings(input: string): Heading[] {
  return parseHeadings(input);
}
