import type { BundledTheme } from "shiki";
import Parser from "./index.ts";

export default async function containers(
  input: string,
  codeTheme: BundledTheme,
): Promise<HastElement> {
  const lines = input.split("\n");
  const type = lines[0].split(" ")[1];

  const values = lines[0].split(" ").slice(2).join(" ");
  let valuesCursor = 0;
  let title = "";
  for (
    valuesCursor;
    valuesCursor < values.length && values[valuesCursor] !== "{";
    valuesCursor++
  ) {
    title += values[valuesCursor];
  }
  title = title.trim() || type.toUpperCase();

  let attr = "";
  for (
    valuesCursor++;
    valuesCursor < values.length && values[valuesCursor] !== "}";
    valuesCursor++
  ) {
    attr += values[valuesCursor];
  }

  const href = attr.split(" ").find((a) => a.startsWith("href")) || "";
  const className = attr.split(" ").find((a) => a.startsWith("class")) || "";
  const clearAttr = attr.replace(href, "").replace(className, "").trim();

  let attrObject: Record<string, string> = {};
  if (clearAttr) {
    for (const a of clearAttr.split(" ")) {
      const [key, val] = a.split("=");
      attrObject[key] = val.slice(1, -1);
    }
  }

  const content = lines.slice(1, -1).join("\n").trim();

  return {
    type: "element",
    tagName: "div",
    properties: {
      class: `znak-container ${type}${className && ` ${className.split("=")[1].slice(1, -1)}`}`,
      ...attrObject,
    },
    children: [
      {
        type: "element",
        tagName: "p",
        properties: { class: `${type}-heading` },
        children: [
          {
            type: "element",
            tagName: "b",
            children: [
              href
                ? {
                    type: "element",
                    tagName: "a",
                    properties: {
                      href: href.split("=")[1].slice(1, -1) || "",
                      target: "_blank",
                      rel: "noopener noreferrer",
                    },
                    children: [{ type: "text", value: title }],
                  }
                : { type: "text", value: title },
            ],
          },
        ],
      },
      ...(await new Parser(content, codeTheme).parse()),
    ],
  };
}
