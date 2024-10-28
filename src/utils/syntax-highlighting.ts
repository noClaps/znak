import { type BundledLanguage } from "shiki";
import type { CodeTheme } from "../../index.ts";
import { codeToHtml } from "shiki/bundle/web";

export async function highlightSyntax(
  code: string,
  theme: CodeTheme,
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
