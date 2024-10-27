import { inlineFormatting } from "./inline-formatting.ts";

export function images(line: string): HastElement {
  let imageTitle = "";
  let imageURL = "";
  let isInsideNestedBlock = false;
  // Start with cursor inside image title
  for (let cursor = 2; cursor < line.length; cursor++) {
    while (line[cursor] !== "]" || isInsideNestedBlock) {
      if (line[cursor] === "[") {
        isInsideNestedBlock = true;
      }
      if (line[cursor] === "]" && isInsideNestedBlock) {
        isInsideNestedBlock = false;
      }
      imageTitle += line[cursor];
      cursor++;
    }

    // Move cursor inside image URL
    cursor += 2;
    while (line[cursor] !== ")") {
      imageURL += line[cursor];
      cursor++;
    }
  }
  return {
    type: "element",
    tagName: "figure",
    children: [
      {
        type: "element",
        tagName: "img",
        children: [],
        properties: new Map([
          ["src", `${imageURL}`],
          ["alt", `${imageTitle}`],
        ]),
      },
      {
        type: "element",
        tagName: "figcaption",
        children: inlineFormatting(imageTitle),
      },
    ],
  };
}
