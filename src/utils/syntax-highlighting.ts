import { codeToHast, type BundledLanguage, type BundledTheme } from "shiki";

export async function highlightSyntax(
  code: string,
  theme: BundledTheme,
  lang?: BundledLanguage,
) {
  return (await codeToHast(code, { lang: lang || "plaintext", theme }))
    .children[0] as HastElement;
}
