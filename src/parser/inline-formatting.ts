import { escapeHTML } from "../utils/escape-html.ts";
import { renderMath } from "../utils/math.ts";

export function inlineFormatting(line: string) {
  const contents: (HastText | HastElement)[] = [];
  let buffer = "";
  for (let cursor = 0; cursor < line.length; cursor++) {
    // Escape characters
    if (line[cursor] === "\\") {
      buffer += line[++cursor];
      continue;
    }

    // Bold (**)
    if (
      line[cursor] + line[cursor + 1] === "**" &&
      line.slice(cursor + 2).includes("**")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfBold = line.indexOf("**", cursor + 2);
      const boldBuffer = line.slice(cursor + 2, nextIndexOfBold);

      if (!boldBuffer) {
        contents.push({ type: "text", value: "****" });
      } else {
        contents.push({
          type: "element",
          tagName: "strong",
          children: inlineFormatting(boldBuffer),
        });
      }

      cursor = nextIndexOfBold + 1;
      continue;
    }

    // Italics (_)
    if (line[cursor] === "_" && line.slice(cursor + 1).includes("_")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfItalic = line.indexOf("_", cursor + 1);
      const italicBuffer = line.slice(cursor + 1, nextIndexOfItalic);

      if (!italicBuffer) {
        contents.push({ type: "text", value: "__" });
      } else {
        contents.push({
          type: "element",
          tagName: "em",
          children: inlineFormatting(italicBuffer),
        });
      }

      cursor = nextIndexOfItalic;
      continue;
    }

    // Code (`)
    if (line[cursor] === "`" && line.slice(cursor + 1).includes("`")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfCode = line.indexOf("`", cursor + 1);
      const codeBuffer = line.slice(cursor + 1, nextIndexOfCode);

      if (!codeBuffer) {
        contents.push({ type: "text", value: "``" });
      } else {
        // Code content is not formatted
        contents.push({
          type: "element",
          tagName: "code",
          children: [
            {
              type: "text",
              value: escapeHTML(codeBuffer),
            },
          ],
        });
      }

      cursor = nextIndexOfCode;
      continue;
    }

    // Strikethrough (~~)
    if (
      line[cursor] + line[cursor + 1] === "~~" &&
      line.slice(cursor + 2).includes("~~")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfStrikethrough = line.indexOf("~~", cursor + 2);
      const strikethroughBuffer = line.slice(
        cursor + 2,
        nextIndexOfStrikethrough,
      );

      if (!strikethroughBuffer) {
        contents.push({ type: "text", value: "~~~~" });
      } else {
        contents.push({
          type: "element",
          tagName: "s",
          children: inlineFormatting(strikethroughBuffer),
        });
      }

      cursor = nextIndexOfStrikethrough + 1;
      continue;
    }

    // Highlight (==)
    if (
      line[cursor] + line[cursor + 1] === "==" &&
      line.slice(cursor + 2).includes("==")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfHighlight = line.indexOf("==", cursor + 2);
      const highlightBuffer = line.slice(cursor + 2, nextIndexOfHighlight);

      if (!highlightBuffer) {
        contents.push({ type: "text", value: "====" });
      } else {
        contents.push({
          type: "element",
          tagName: "mark",
          children: inlineFormatting(highlightBuffer),
        });
      }

      cursor = nextIndexOfHighlight + 1;
      continue;
    }

    // Subscript (~)
    if (line[cursor] === "~" && line.slice(cursor + 1).includes("~")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfSubscript = line.indexOf("~", cursor + 1);
      const subscriptBuffer = line.slice(cursor + 1, nextIndexOfSubscript);

      if (!subscriptBuffer) {
        contents.push({ type: "text", value: "~~" });
      } else {
        contents.push({
          type: "element",
          tagName: "sub",
          children: inlineFormatting(subscriptBuffer),
        });
      }

      cursor = nextIndexOfSubscript;
      continue;
    }

    // Superscript (^)
    if (line[cursor] === "^" && line.slice(cursor + 1).includes("^")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfSuperscript = line.indexOf("^", cursor + 1);
      const superscriptBuffer = line.slice(cursor + 1, nextIndexOfSuperscript);

      if (!superscriptBuffer) {
        contents.push({ type: "text", value: "^^" });
      } else {
        contents.push({
          type: "element",
          tagName: "sup",
          children: inlineFormatting(superscriptBuffer),
        });
      }

      cursor = nextIndexOfSuperscript;
      continue;
    }

    // Links
    if (
      line[cursor] === "[" &&
      line.slice(cursor).includes("](") &&
      line.slice(cursor).includes(")")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor inside link title
      let linkTitle = "";
      let isInsideNestedBlock = false;
      for (cursor++; line[cursor] !== "]" || isInsideNestedBlock; cursor++) {
        if (line[cursor] === "[") {
          isInsideNestedBlock = true;
        }
        if (line[cursor] === "]") {
          isInsideNestedBlock = false;
        }
        linkTitle += line[cursor];
      }

      // Move cursor inside link URL
      let linkURL = "";
      let isInsideLink = false;

      for (cursor += 2; line[cursor] !== ")" || isInsideLink; cursor++) {
        if (line[cursor] === "<") {
          isInsideLink = true;
          continue;
        }
        if (line[cursor] === ">") {
          isInsideLink = false;
          continue;
        }
        linkURL += line[cursor];
      }

      if (!linkTitle || !linkURL) {
        contents.push({ type: "text", value: `[${linkTitle}](${linkURL})` });
      } else {
        contents.push({
          type: "element",
          tagName: "a",
          children: inlineFormatting(linkTitle),
          properties: new Map([["href", `${linkURL}`]]),
        });
      }

      continue;
    }

    // Inline math ($$)
    if (
      line[cursor] + line[cursor + 1] === "$$" &&
      line.slice(cursor + 2).includes("$$")
    ) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      const nextIndexOfMath = line.indexOf("$$", cursor + 2);
      const mathBuffer = line.slice(cursor + 2, nextIndexOfMath);

      if (!mathBuffer) {
        contents.push({ type: "text", value: "$$$$" });
      } else {
        contents.push(renderMath(mathBuffer, false));
      }

      cursor = nextIndexOfMath + 1;
      continue;
    }

    // Default
    buffer += line[cursor];
  }

  if (buffer) {
    contents.push({ type: "text", value: buffer });
  }

  return contents;
}
