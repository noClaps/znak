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

  const attr =
    nextIndexOfAttr === -1 ? "" : values.slice(nextIndexOfAttr + 1, -1);
  const title = values.replace(`{${attr}}`, "").trim() || type.toUpperCase();

  const attrObject = new Map<string, string>();
  if (attr) {
    for (const a of attr.split(" ")) {
      const [key, val] = a.split("=");
      attrObject.set(key, val.slice(1, -1));
    }
  }

  attrObject.set(
    "class",
    `znak-container ${type} ${attrObject.get("class") ?? ""}`.trim(),
  );

  const href = attrObject.get("href");
  attrObject.delete("href");

  const content = body.slice(0, -1).join("\n").trim();

  return {
    type: "element",
    tagName: "div",
    properties: attrObject,
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
                      ["href", href],
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
