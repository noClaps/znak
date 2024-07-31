import inlineFormatting from "./inline-formatting.ts";

export default function tables(input: string): Token {
  const lines = input.trim().split("\n");
  const thead = lines[0]
    .split("|")
    .filter((c) => c)
    .map((col) => col.trim());
  const alignments = lines[1]
    .split("|")
    .filter((c) => c)
    .map((col) => {
      const trimmedCol = col.trim();
      if (trimmedCol.startsWith(":") && trimmedCol.endsWith(":"))
        return "center";
      if (trimmedCol.endsWith(":")) return "right";
      if (trimmedCol.startsWith(":")) return "left";
      return "";
    });

  const token: Token = {
    element: "table",
    contents: [
      {
        element: "thead",
        contents: [
          {
            element: "tr",
            contents: thead.map((th, index) => ({
              element: "th",
              contents: inlineFormatting(th),
              attributes: {
                align: alignments[index],
              },
            })),
          },
        ],
      },
      {
        element: "tbody",
        contents: lines.slice(2).map((line) => ({
          element: "tr",
          contents: line
            .split("|")
            .filter((c) => c)
            .map((col, index) => ({
              element: "td",
              contents: inlineFormatting(col.trim()),
              attributes: { align: alignments[index] },
            })),
        })),
      },
    ],
  };

  return token;
}
