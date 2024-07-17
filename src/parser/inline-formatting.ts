export default function inlineFormatting(line: string): (string | Token)[] {
  const contents: (string | Token)[] = [];
  let buffer = "";
  for (let cursor = 0; cursor < line.length; cursor++) {
    // Bold (**)
    if (
      line[cursor] + line[cursor + 1] === "**" &&
      line.slice(cursor + 2).includes("**")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside bold block
      cursor += 2;
      let boldBuffer = "";
      while (line[cursor] + line[cursor + 1] !== "**") {
        boldBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "strong",
        contents: inlineFormatting(boldBuffer),
      });
      cursor++;
      continue;
    }

    // Italics (_)
    if (line[cursor] === "_" && line.slice(cursor + 1).includes("_")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside italic block
      cursor++;
      let italicBuffer = "";
      while (line[cursor] !== "_") {
        italicBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "em",
        contents: inlineFormatting(italicBuffer),
      });
      continue;
    }

    // Code (`)
    if (line[cursor] === "`" && line.slice(cursor + 1).includes("`")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside code block
      cursor++;
      let codeBuffer = "";
      while (line[cursor] !== "`") {
        codeBuffer += line[cursor];
        cursor++;
      }
      // Code content is not formatted
      contents.push({ element: "code", contents: [codeBuffer] });
      continue;
    }

    // Strikethrough (~~)
    if (
      line[cursor] + line[cursor + 1] === "~~" &&
      line.slice(cursor + 2).includes("~~")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside bold block
      cursor += 2;
      let strikethroughBuffer = "";
      while (line[cursor] + line[cursor + 1] !== "~~") {
        strikethroughBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "s",
        contents: inlineFormatting(strikethroughBuffer),
      });
      cursor++;
      continue;
    }

    // Highlight (==)
    if (
      line[cursor] + line[cursor + 1] === "==" &&
      line.slice(cursor + 2).includes("==")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside bold block
      cursor += 2;
      let highlightBuffer = "";
      while (line[cursor] + line[cursor + 1] !== "==") {
        highlightBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "mark",
        contents: inlineFormatting(highlightBuffer),
      });
      cursor++;
      continue;
    }

    // Subscript (~)
    if (line[cursor] === "~" && line.slice(cursor + 1).includes("~")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside italic block
      cursor++;
      let subscriptBuffer = "";
      while (line[cursor] !== "~") {
        subscriptBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "sub",
        contents: inlineFormatting(subscriptBuffer),
      });
      continue;
    }

    // Superscript (^)
    if (line[cursor] === "^" && line.slice(cursor + 1).includes("^")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor to inside italic block
      cursor++;
      let superscriptBuffer = "";
      while (line[cursor] !== "^") {
        superscriptBuffer += line[cursor];
        cursor++;
      }
      contents.push({
        element: "sup",
        contents: inlineFormatting(superscriptBuffer),
      });
      continue;
    }

    // Links
    if (line[cursor] === "[" && line.slice(cursor).match(/^\[.*\]\(.+\)/gm)) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      // Move cursor inside link title
      cursor++;
      let linkTitle = "";
      while (line[cursor] !== "]") {
        linkTitle += line[cursor];
        cursor++;
      }

      // Move cursor inside link URL
      cursor += 2;
      let linkURL = "";
      while (line[cursor] !== ")") {
        linkURL += line[cursor];
        cursor++;
      }
      contents.push({
        element: "a",
        contents: inlineFormatting(linkTitle),
        attributes: { href: linkURL },
      });
      continue;
    }

    // Inline math ($)
    if (line[cursor] === "$" && line.slice(cursor).includes("$")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push(buffer);
        buffer = "";
      }

      let math = "";
      for (cursor++; line[cursor] !== "$"; cursor++) {
        math += line[cursor];
      }
      contents.push({
        element: "math",
        contents: [math],
        attributes: { "data-display": "inline" },
      });

      continue;
    }

    // Default
    buffer += line[cursor];
  }

  if (buffer) {
    contents.push(buffer);
  }

  return contents;
}
