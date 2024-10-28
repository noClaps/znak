import { inlineFormatting } from "./inline-formatting.ts";

export function tables(input: string): HastElement {
  const [headRow, alignmentsRow, ...tbody] = input.trim().split("\n");
  const thead = headRow
    .slice(1, -1)
    .split("|")
    .map((col) => col.trim());
  const alignments = alignmentsRow
    .slice(1, -1)
    .split("|")
    .map((col) => {
      const trimmedCol = col.trim();
      if (trimmedCol.startsWith(":") && trimmedCol.endsWith(":"))
        return "center";
      if (trimmedCol.endsWith(":")) return "right";
      if (trimmedCol.startsWith(":")) return "left";
      return "";
    });

  const token: HastElement = {
    type: "element",
    tagName: "table",
    children: [
      {
        type: "element",
        tagName: "thead",
        children: [
          {
            type: "element",
            tagName: "tr",
            children: thead.map((th, index) => ({
              type: "element",
              tagName: "th",
              children: inlineFormatting(th),
              properties: new Map([["align", `${alignments[index]}`]]),
            })),
          },
        ],
      },
      {
        type: "element",
        tagName: "tbody",
        children: tbody.map((line) => ({
          type: "element",
          tagName: "tr",
          children: line
            .slice(1, -1)
            .split("|")
            .map((col, index) => ({
              type: "element",
              tagName: "td",
              children: inlineFormatting(col.trim()),
              properties: new Map([["align", `${alignments[index]}`]]),
            })),
        })),
      },
    ],
  };

  return token;
}
