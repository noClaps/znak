import type { CodeTheme } from "../../index.ts";
import { parse } from "./index.ts";

export async function containers(
  input: string,
  codeTheme: CodeTheme,
): Promise<HastElement> {
  const [head, ...body] = input.split("\n");
  const [_, type, ...meta] = head.split(" ");

  const values = meta.join(" ");
  const nextIndexOfAttr = values.indexOf("{");

  const title =
    values
      .slice(0, nextIndexOfAttr === -1 ? undefined : nextIndexOfAttr)
      .trim() || type.toUpperCase();
  const attr =
    nextIndexOfAttr === -1 ? "" : values.slice(nextIndexOfAttr + 1, -1);

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

  const content = body.slice(0, -1).join("\n").trim();

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
