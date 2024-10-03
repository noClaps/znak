import {
	createWasmOnigEngine,
	createHighlighterCoreSync,
	type BundledLanguage,
	type BundledTheme,
	bundledLanguagesInfo,
	type LanguageRegistration,
	type MaybeArray,
	type ThemeRegistration,
	bundledThemesInfo,
} from "shiki";

const langs: MaybeArray<LanguageRegistration>[] = [];
for (const bl of bundledLanguagesInfo) {
	const { default: lang } = await bl.import();
	langs.push(lang);
}

const themes: MaybeArray<ThemeRegistration>[] = [];
for (const bt of bundledThemesInfo) {
	const { default: theme } = await bt.import();
	themes.push(theme);
}

const engine = await createWasmOnigEngine(import("shiki/wasm"));
const shiki = createHighlighterCoreSync({
	langs,
	themes,
	engine,
});

export function highlightSyntax(
	code: string,
	theme: BundledTheme,
	lang?: BundledLanguage,
) {
	return shiki.codeToHast(code, { lang: lang || "plaintext", theme })
		.children[0] as HastElement;
}
