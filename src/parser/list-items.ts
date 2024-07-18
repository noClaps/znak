import parser from "./index";

export function orderedListItems(input: string): Token[] {
  const lines = input
    .split(/^\d+\. /gm)
    .filter((l) => l)
    .map((l) => l.trim());
  const tokens: Token[] = lines.map((line) => ({
    element: "li",
    contents: parser(
      line
        .split("\n")
        .map((l) => (l.startsWith("   ") ? l.replace("   ", "") : l))
        .join("\n")
    ),
  }));
  return tokens;
}

export function unorderedListItems(input: string): Token[] {
  const lines = input
    .split(/^- /gm)
    .filter((l) => l)
    .map((l) => l.trim());
  const tokens: Token[] = lines.map((line) => ({
    element: "li",
    contents: parser(
      line
        .split("\n")
        .map((l) => (l.startsWith("  ") ? l.replace("  ", "") : l))
        .join("\n")
    ),
  }));
  return tokens;
}
