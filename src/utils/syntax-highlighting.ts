import {
	type BundledLanguage,
	bundledLanguages,
	type BundledTheme,
	bundledThemes,
	createHighlighter,
} from "shiki";

const shiki = await createHighlighter({
	themes: Object.keys(bundledThemes),
	langs: Object.keys(bundledLanguages),
});

export function highlightSyntax(
	code: string,
	theme: BundledTheme,
	lang?: BundledLanguage,
): HastText {
	return {
		type: "text",
		value: shiki.codeToHtml(code, {
			lang: lang || "plaintext",
			theme,
		}),
	};
}
