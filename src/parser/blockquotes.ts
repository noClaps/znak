import type { BundledTheme } from "shiki";
import { parse } from "./index.ts";

export function blockquotes(
	input: string,
	codeTheme: BundledTheme,
): HastElement {
	const lines = input
		.split("\n")
		.map((line) => line.slice(1).trim())
		.join("\n");

	return {
		type: "element",
		tagName: "blockquote",
		children: parse(lines, codeTheme),
	};
}
