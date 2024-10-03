import inlineFormatting from "./inline-formatting.ts";
import type { Slugger } from "../utils/slugger.ts";

export default function headings(line: string, slugger: Slugger): HastElement {
	let level = 0;
	let cursor = 0;
	let char = line[cursor];

	while (char === "#") {
		level++;
		cursor++;
		char = line[cursor];
	}

	const content = line.slice(cursor + 1);
	const slug = slugger.slug(content, level);
	return {
		type: "element",
		tagName: `h${level}`,
		children: inlineFormatting(content),
		properties: { id: slug },
	};
}
