import inlineFormatting from "./inline-formatting";
import GitHubSlugger from "github-slugger";

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
  const slug = new GitHubSlugger().slug(content);
  return {
    element: `h${level}`,
    contents: inlineFormatting(content),
    attributes: { id: slug },
  };
}
