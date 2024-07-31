import inlineFormatting from "./inline-formatting.ts";

export default function images(line: string): Token {
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
    element: "figure",
    contents: [
      {
        element: "img",
        contents: [],
        attributes: { src: imageURL, alt: imageTitle },
      },
      { element: "figcaption", contents: inlineFormatting(imageTitle) },
    ],
  };
}
