import { inlineFormatting } from "./inline-formatting.ts";

export function tables(input: string): HastElement {
	const lines = input.trim().split("\n");
	const thead = lines[0]
		.split("|")
		.filter((c) => c)
		.map((col) => col.trim());
	const alignments = lines[1]
		.split("|")
		.filter((c) => c)
		.map((col) => {
			const trimmedCol = col.trim();
			if (trimmedCol.startsWith(":") && trimmedCol.endsWith(":"))
				return "center";
			if (trimmedCol.endsWith(":")) return "right";
			if (trimmedCol.startsWith(":")) return "left";
			return "";
		});

	const token: HastElement = {
		type: "element",
		tagName: "table",
		children: [
			{
				type: "element",
				tagName: "thead",
				children: [
					{
						type: "element",
						tagName: "tr",
						children: thead.map((th, index) => ({
							type: "element",
							tagName: "th",
							children: inlineFormatting(th),
							properties: {
								align: alignments[index],
							},
						})),
					},
				],
			},
			{
				type: "element",
				tagName: "tbody",
				children: lines.slice(2).map((line) => ({
					type: "element",
					tagName: "tr",
					children: line
						.split("|")
						.filter((c) => c)
						.map((col, index) => ({
							type: "element",
							tagName: "td",
							children: inlineFormatting(col.trim()),
							properties: { align: alignments[index] },
						})),
				})),
			},
		],
	};

	return token;
}
