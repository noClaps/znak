import katex from "katex";
import { codeToHtml, type BundledTheme } from "shiki";

export default async function renderer(
  token: HastText | HastToken | HastElement,
  codeTheme: BundledTheme,
): Promise<string> {
  switch (token.type) {
    case "text":
      return token.value;

    case "element":
      const attributeList = token.properties
        ? Object.keys(token.properties)
            .map((key) => ` ${key}="${token.properties![key]}"`)
            .join("")
        : "";

      let contents = "";
      for (const item of token.children) {
        contents += await renderer(item, codeTheme);
      }

      if (token.children.length === 0) {
        return `<${token.tagName}${attributeList} />`;
      }

      return `<${token.tagName}${attributeList}>${contents}</${token.tagName}>`;

    case "token":
      switch (token.tokenName) {
        case "math": {
          const firstChild = token.children[0];
          if (!firstChild || firstChild.type !== "text") {
            throw new Error(`Invalid child for math token: ${firstChild}`);
          }
          if (!token.properties) {
            throw new Error(`Math token has no properties: ${token}`);
          }

          return katex.renderToString(firstChild.value, {
            displayMode: token.properties["data-display"] === "block",
          });
        }

        case "code-block": {
          const firstChild = token.children[0];
          if (!firstChild || firstChild.type !== "text") {
            throw new Error(
              `Invalid child for code-block token: ${firstChild}`,
            );
          }
          if (!token.properties) {
            throw new Error(`Code-block token has no properties: ${token}`);
          }

          return await codeToHtml(firstChild.value, {
            lang: token.properties["data-lang"] || "plaintext",
            theme: codeTheme,
          });
        }

        case "container":
          if (!token.properties) {
            throw new Error(`Container token has no properties: ${token}`);
          }
          const type = token.properties.type;
          const title = token.properties.title || type.toUpperCase();
          const attr = token.properties.attr;
          const href = attr.split(" ").find((a) => a.startsWith("href")) || "";
          const attrWithoutHref = attr.replace(href, "");

          let contents = "";
          for (const item of token.children) {
            contents += await renderer(item, codeTheme);
          }
          return `<div class="znak-container ${type}"${attrWithoutHref}><p class="${type}-heading"><b>${
            href
              ? `<a ${href} target="_blank" rel="noopener noreferrer">${title}</a>`
              : title
          }</b></p>${contents}</div>`;

        default:
          throw new Error(`Unrecognised token: ${token}`);
      }
  }
}
