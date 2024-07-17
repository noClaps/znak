import katex from "katex";
import { codeToHtml, type BundledTheme } from "shiki";

export default async function renderer(
  token: Token,
  codeTheme: BundledTheme = "github-dark",
): Promise<string> {
  switch (token.element) {
    case "math": {
      // Math
      return katex.renderToString(token.contents[0].toString(), {
        displayMode: token.attributes!["data-display"] === "block",
      });
    }

    case "code": {
      if (!token.attributes) {
        // Inline code or no language set
        return `<code>${token.contents}</code>`;
      }

      const attributeList = Object.keys(token.attributes)
        .map((key) => `${key}="${token.attributes![key]}"`)
        .join(" ");
      return `<code ${attributeList}>${await codeToHtml(
        token.contents[0].toString(),
        {
          lang: token.attributes["data-lang"] || "plaintext",
          theme: codeTheme,
          structure: "inline",
        },
      )}</code>`;
    }

    case "raw": {
      return token.contents[0].toString();
    }

    default: {
      const attributeList = token.attributes
        ? Object.keys(token.attributes)
            .map((key) => `${key}="${token.attributes![key]}"`)
            .join(" ")
        : "";
      let contents: string = "";

      for (const item of token.contents) {
        if (typeof item === "string") {
          contents += item;
        } else {
          contents += await renderer(item);
        }
      }

      return `<${token.element} ${attributeList}>${contents}</${token.element}>`;
    }
  }
}
