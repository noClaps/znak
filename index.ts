import type { ThemeRegistration } from "shiki";
import { parse, parseHeadings } from "./src/parser/index.ts";
import { renderer } from "./src/renderer.ts";
import type { Heading } from "./src/utils/slugger.ts";
import githubDark from "shiki/themes/github-dark.mjs";

/**
 * A code theme. This is taken from Shiki's ThemeRegistration type. You can
 * use it to make your own themes and pass them in to the `render()` function.
 *
 * @example
 * ```ts
 * import { type CodeTheme, render } from "@noclaps/znak";
 * const myTheme: CodeTheme = {
 *   // ...
 * };
 *
 * await render(input, myTheme);
 * ```
 * You can read about how to create themes in
 * [Shiki's documentation](https://shiki.style/guide/load-theme).
 */
export type CodeTheme = ThemeRegistration;
export { type Heading };

/**
 * A function that renders the input text to HTML.
 *
 * @param [input] The input text to be converted to HTML. This can be from a
 * Markdown file as long as the syntax is supported by Znak. See the
 * [documentation](https://github.com/noClaps/znak#syntax) for the supported
 * syntax.
 *
 * @param [codeTheme] The theme for code blocks. This is set to GitHub Dark
 * by default, and can be set to any of the syntax highlighting themes
 * included in [Shiki](https://shiki.style/themes). To use a different theme,
 * add Shiki as a dependency and import the theme using:
 * ```ts
 * import theme from "shiki/themes/[theme].mjs
 *
 * await render(input, theme)
 * ```
 *
 * @returns An HTML string created from the input text.
 */
export async function render(
	input: string,
	codeTheme: CodeTheme = githubDark,
): Promise<string> {
	const parserOutput = await parse(input, codeTheme);
	return parserOutput.map((po) => renderer(po)).join("");
}

/**
 * A function that returns the headings in the given input text.
 *
 * @param [input] The input text to extract the headings from. This can be from
 * a Markdown file as long as the syntax is supported by Znak. See the
 * [documentation](https://github.com/noClaps/znak#syntax) for the supported
 * syntax.
 *
 * @returns A list of headings in the given input text.
 */
export function headings(input: string): Heading[] {
	return parseHeadings(input);
}
