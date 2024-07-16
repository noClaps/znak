import inlineFormatting from "./inline-formatting";

export default function headings(line: string): Token {
  let level = 0;
  let cursor = 0;
  let char = line[cursor];

  while (char === "#") {
    level++;
    cursor++;
    char = line[cursor];
  }

  const content = line.slice(cursor + 1);
  return { element: `h${level}`, contents: inlineFormatting(content) };
}
