import type { BundledTheme } from "shiki";
import parser from "./index.ts";

export default async function blockquotes(
  input: string,
  codeTheme: BundledTheme,
): Promise<HastElement> {
  const lines = input
    .split("\n")
    .map((line) => line.slice(1).trim())
    .join("\n");

  return {
    type: "element",
    tagName: "blockquote",
    children: await parser(lines, codeTheme),
  };
}
