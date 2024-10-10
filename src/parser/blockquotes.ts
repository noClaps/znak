import type { CodeTheme } from "../../index.ts";
import { parse } from "./index.ts";

export async function blockquotes(
	input: string,
	codeTheme: CodeTheme,
): Promise<HastElement> {
	const lines = input
		.split("\n")
		.map((line) => line.slice(1).trim())
		.join("\n");

	return {
		type: "element",
		tagName: "blockquote",
		children: await parse(lines, codeTheme),
	};
}
