import katex from "katex";

export function renderMath(input: string, isBlock: boolean): HastText {
	return {
		type: "text",
		value: katex.renderToString(input, { displayMode: isBlock }),
	};
}
