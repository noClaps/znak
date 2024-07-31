import parser from "./index.ts";

export function orderedListItems(input: string): Token[] {
  const lines = input
    .split(/^\d+\. /gm)
    .filter((l) => l)
    .map((l) => l.trim());

  const tokens: Token[] = lines.map((line) => {
    const segments = line.split("\n");
    if (segments.length === 1) return { element: "li", contents: parser(line) };

    return {
      element: "li",
      contents: parser(
        `${segments[0]}\n\n${segments
          .slice(1)
          .map((l) => l.replace(/^(   |\t)/m, ""))
          .join("\n")}`
      ),
    };
  });
  return tokens;
}

export function unorderedListItems(input: string): Token[] {
  const lines = input
    .split(/^- /gm)
    .filter((l) => l)
    .map((l) => l.trim());
  const tokens: Token[] = lines.map((line) => {
    const segments = line.split("\n");
    if (segments.length === 1) return { element: "li", contents: parser(line) };

    return {
      element: "li",
      contents: parser(
        `${segments[0]}\n\n${segments
          .slice(1)
          .map((l) => l.replace(/^(  |\t)/m, ""))
          .join("\n")}`
      ),
    };
  });
  return tokens;
}
