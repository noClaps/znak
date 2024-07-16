import inlineFormatting from "./inline-formatting";

export default function images(line: string): Token {
  let imageTitle = "";
  let imageURL = "";
  // Start with cursor inside image title
  for (let cursor = 2; cursor < line.length; cursor++) {
    while (line[cursor] !== "]") {
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
