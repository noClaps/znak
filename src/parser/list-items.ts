import type { BundledTheme } from "shiki";
import parser from "./index.ts";

export function orderedListItems(input: string, codeTheme: BundledTheme) {
  const lines = input
    .split(/^\d+\. /gm)
    .filter((l) => l)
    .map((l) => l.trim());

  const tokens: HastElement[] = lines.map((line) => {
    const segments = line.split("\n");
    if (segments.length === 1) {
      return {
        type: "element",
        tagName: "li",
        children: parser(line, codeTheme),
      };
    }

    return {
      type: "element",
      tagName: "li",
      children: parser(
        `${segments[0]}\n\n${segments
          .slice(1)
          .map((l) => l.replace(/^(   |\t)/m, ""))
          .join("\n")}`,
        codeTheme,
      ),
    };
  });

  return tokens;
}

export function unorderedListItems(input: string, codeTheme: BundledTheme) {
  const lines = input
    .split(/^- /gm)
    .filter((l) => l)
    .map((l) => l.trim());
  const tokens: HastElement[] = lines.map((line) => {
    const segments = line.split("\n");
    if (segments.length === 1) {
      return {
        type: "element",
        tagName: "li",
        children: parser(line, codeTheme),
      };
    }

    return {
      type: "element",
      tagName: "li",
      children: parser(
        `${segments[0]}\n\n${segments
          .slice(1)
          .map((l) => l.replace(/^(  |\t)/m, ""))
          .join("\n")}`,
        codeTheme,
      ),
    };
  });

  return tokens;
}
