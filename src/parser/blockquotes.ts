import parser from "./index.ts";

export default function blockquotes(input: string): HastElement {
  const lines = input
    .split("\n")
    .map((line) => line.slice(1).trim())
    .join("\n");

  return { type: "element", tagName: "blockquote", children: parser(lines) };
}
