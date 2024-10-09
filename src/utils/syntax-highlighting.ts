import { codeToHtml, type BundledTheme, type BundledLanguage } from "shiki";

export async function highlightSyntax(
	code: string,
	theme: BundledTheme,
	lang?: BundledLanguage,
): Promise<HastText> {
	return {
		type: "text",
		value: await codeToHtml(code, {
			lang: lang || "plaintext",
			theme,
		}),
	};
}
