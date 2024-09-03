import type { BundledTheme } from "shiki";
import Parser from "./index.ts";

export async function orderedListItems(input: string, codeTheme: BundledTheme) {
  const lines = input
    .split(/^\d+\. /gm)
    .filter((l) => l)
    .map((l) => l.trim());

  const tokens: HastElement[] = await Promise.all(
    lines.map(async (line) => {
      const segments = line.split("\n");
      if (segments.length === 1) {
        return {
          type: "element",
          tagName: "li",
          children: await new Parser(line, codeTheme).parse(),
        };
      }

      return {
        type: "element",
        tagName: "li",
        children: await new Parser(
          `${segments[0]}\n\n${segments
            .slice(1)
            .map((l) => l.replace(/^(   |\t)/m, ""))
            .join("\n")}`,
          codeTheme,
        ).parse(),
      };
    }),
  );
  return tokens;
}

export async function unorderedListItems(
  input: string,
  codeTheme: BundledTheme,
) {
  const lines = input
    .split(/^- /gm)
    .filter((l) => l)
    .map((l) => l.trim());
  const tokens: HastElement[] = await Promise.all(
    lines.map(async (line) => {
      const segments = line.split("\n");
      if (segments.length === 1) {
        return {
          type: "element",
          tagName: "li",
          children: await new Parser(line, codeTheme).parse(),
        };
      }

      return {
        type: "element",
        tagName: "li",
        children: await new Parser(
          `${segments[0]}\n\n${segments
            .slice(1)
            .map((l) => l.replace(/^(  |\t)/m, ""))
            .join("\n")}`,
          codeTheme,
        ).parse(),
      };
    }),
  );
  return tokens;
}
