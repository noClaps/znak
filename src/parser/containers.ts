import type { CodeTheme } from "../../index.ts";
import { parse } from "./index.ts";

export async function containers(
  input: string,
  codeTheme: CodeTheme,
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

  const attrObject = new Map<string, string>();
  if (clearAttr) {
    for (const a of clearAttr.split(" ")) {
      const [key, val] = a.split("=");
      attrObject.set(key, val.slice(1, -1));
    }
  }

  const content = lines.slice(1, -1).join("\n").trim();

  return {
    type: "element",
    tagName: "div",
    properties: new Map([
      [
        "class",
        `znak-container ${type}${className && ` ${className.split("=")[1].slice(1, -1)}`}`,
      ],
      ...attrObject,
    ]),
    children: [
      {
        type: "element",
        tagName: "p",
        properties: new Map([["class", `${type}-heading`]]),
        children: [
          {
            type: "element",
            tagName: "b",
            children: [
              href
                ? {
                    type: "element",
                    tagName: "a",
                    properties: new Map([
                      ["href", href.split("=")[1].slice(1, -1) || ""],
                      ["target", "_blank"],
                      ["rel", "noopener noreferrer"],
                    ]),
                    children: [{ type: "text", value: title }],
                  }
                : { type: "text", value: title },
            ],
          },
        ],
      },
      ...(await parse(content, codeTheme)),
    ],
  };
}
