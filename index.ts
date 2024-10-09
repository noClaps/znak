import type { BundledTheme } from "shiki";
import { parse, parseHeadings } from "./src/parser/index.ts";
import { renderer } from "./src/renderer.ts";
import type { Heading } from "./src/utils/slugger.ts";

/**
 * @param [input] The input text to be converted to HTML. This can be from a
 * Markdown file as long as the syntax is supported by Znak. See the
 * [documentation](https://github.com/noClaps/znak#syntax) for the supported
 * syntax.
 *
 * @param [codeTheme] The theme for code blocks. This is set to "github-dark"
 * by default, and can be set to any of the syntax highlighting themes
 * included in [Shiki](https://shiki.style/themes).
 *
 * @returns An HTML string created from the input text.
 */
export function render(input: string, codeTheme: BundledTheme = "github-dark") {
	const parserOutput = parse(input, codeTheme);
	return parserOutput.map((po) => renderer(po)).join("");
}

/**
 * A method that returns the headings in the given input text.
 * @returns A list of headings in the given input text.
 */
export function headings(input: string): Heading[] {
	return parseHeadings(input);
}
