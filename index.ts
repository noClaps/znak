import type { BundledTheme } from "shiki";
import parser from "./src/parser";
import renderer from "./src/renderer";

/**
 * A parser for a Markdown-like markup language that supports a smaller subset
 * of Markdown syntax, and is stricter and more opinionated. It has features
 * like syntax highlighting, KaTeX support, and heading IDs built-in.
 *
 * @example
 * ```ts
 * const text = "# Hello World";
 * const outputHTML = await new Znak(text).renderToHTML();
 * // <h1 id="hello-world">Hello World</h1>
 * ```
 */
export default class Znak {
  #md: string;
  #codeTheme: BundledTheme;

  /**
   * @param [input] The input text to be converted to HTML. This can be from a
   * Markdown file as long as the syntax is supported by Znak. See the
   * [documentation](https://github.com/noClaps/znak#syntax) for the supported
   * syntax.
   *
   * @param [codeTheme] The theme for code blocks. This is set to "github-dark"
   * by default, and can be set to any of the syntax highlighting themes
   * included in [Shiki](https://shiki.style/themes).
   */
  constructor(input: string, codeTheme: BundledTheme = "github-dark") {
    this.#md = input;
    this.#codeTheme = codeTheme;
  }

  /**
   * The method that outputs HTML for the given input text.
   * @returns An HTML string created from the input text.
   */
  async renderToHTML(): Promise<string> {
    const parserOutput = parser(this.#md);
    return await Promise.all(
      parserOutput.map(async (po) => await renderer(po, this.#codeTheme))
    ).then((ro) => ro.join(""));
  }
}
