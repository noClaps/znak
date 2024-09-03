import type GitHubSlugger from "github-slugger";
import inlineFormatting from "./inline-formatting.ts";

export default function headings(
  line: string,
  slugger: GitHubSlugger,
): HastElement {
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
    type: "element",
    tagName: `h${level}`,
    children: inlineFormatting(content),
    properties: { id: slug },
  };
}
