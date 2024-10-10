import {
	type BundledLanguage,
	createHighlighterCore,
	createOnigurumaEngine,
} from "shiki";
import type { CodeTheme } from "../../index.ts";

const shiki = await createHighlighterCore({
	themes: [],
	langs: [],
	engine: createOnigurumaEngine(import("shiki/wasm")),
});

export async function highlightSyntax(
	code: string,
	theme: CodeTheme,
	lang?: BundledLanguage,
): Promise<HastText> {
	await shiki.loadTheme(theme);
	if (lang) {
		await shiki.loadLanguage(import(`shiki/langs/${lang}.mjs`));
	}
	return {
		type: "text",
		value: shiki.codeToHtml(code, {
			lang: lang || "plaintext",
			theme,
		}),
	};
}
