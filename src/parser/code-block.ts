import {
	bundledLanguages,
	type BundledLanguage,
	type BundledTheme,
} from "shiki";
import { highlightSyntax } from "../utils/syntax-highlighting.ts";

export default function codeBlock(input: string, codeTheme: BundledTheme) {
	const lines = input.split("\n");
	const language = lines[0].replaceAll("`", "");
	const code = lines.slice(1, -1).join("\n").trim();

	if (language && !(language in bundledLanguages)) {
		console.error(
			`Language not supported by Shiki: ${language}, continuing as plaintext`,
		);
	}

	if (language && language in bundledLanguages) {
		return highlightSyntax(code, codeTheme, language as BundledLanguage);
	}

	return highlightSyntax(code, codeTheme);
}
