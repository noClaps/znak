import {
	type BundledLanguage,
	bundledLanguages,
	createHighlighterCore,
	createOnigurumaEngine,
} from "shiki";
import type { CodeTheme } from "../../index.ts";

const shiki = await createHighlighterCore({
	themes: [],
	langs: Object.values(bundledLanguages),
	engine: createOnigurumaEngine(import("shiki/wasm")),
});

export async function highlightSyntax(
	code: string,
	theme: CodeTheme,
	lang?: BundledLanguage,
): Promise<HastText> {
	await shiki.loadTheme(theme);
	return {
		type: "text",
		value: shiki.codeToHtml(code, {
			lang: lang || "plaintext",
			theme,
		}),
	};
}
