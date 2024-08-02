import katex from "katex";
import { codeToHtml, type BundledTheme } from "shiki";

export default async function renderer(
  token: Token,
  codeTheme: BundledTheme
): Promise<string> {
  switch (token.element) {
    case "math": {
      return katex.renderToString(token.contents[0].toString(), {
        displayMode: token.attributes!["data-display"] === "block",
      });
    }

    case "code-block": {
      return await codeToHtml(token.contents[0].toString(), {
        lang: token.attributes!["data-lang"] || "plaintext",
        theme: codeTheme,
      });
    }

    case "container": {
      const type = token.attributes!.type;
      const title = token.attributes!.title || type.toUpperCase();
      const attr = token.attributes!.attr;
      const href = attr.split(" ").find((a) => a.startsWith("href")) || "";
      const attrWithoutHref = attr.replace(href, "");

      let contents = "";
      for (const item of token.contents) {
        if (typeof item === "string") {
          contents += item;
        } else {
          contents += await renderer(item, codeTheme);
        }
      }

      return `<div class="${type}"${attrWithoutHref}><p class="${type}-heading"><b>${
        href
          ? `<a ${href} target="_blank" rel="noopener noreferrer">${title}</a>`
          : title
      }</b></p>${contents}</div>`;
    }

    case "raw": {
      return token.contents[0].toString();
    }

    default: {
      const attributeList = token.attributes
        ? Object.keys(token.attributes)
            .map((key) => ` ${key}="${token.attributes![key]}"`)
            .join("")
        : "";
      let contents: string = "";

      for (const item of token.contents) {
        if (typeof item === "string") {
          contents += item;
        } else {
          contents += await renderer(item, codeTheme);
        }
      }

      return `<${token.element}${attributeList}>${contents}</${token.element}>`;
    }
  }
}
