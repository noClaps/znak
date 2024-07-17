import inlineFormatting from "./inline-formatting";
import type GitHubSlugger from "github-slugger";

export default function headings(line: string, slugger: GitHubSlugger): Token {
  let level = 0;
  let cursor = 0;
  let char = line[cursor];

  while (char === "#") {
    level++;
    cursor++;
    char = line[cursor];
  }

  const content = line.slice(cursor + 1);
  const slug = slugger.slug(content);
  return {
    element: `h${level}`,
    contents: inlineFormatting(content),
    attributes: { id: slug },
  };
}
