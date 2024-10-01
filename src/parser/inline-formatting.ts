import { escapeHTML } from "../utils/escape-html.ts";
import { renderMath } from "../utils/math.ts";

export default function inlineFormatting(line: string) {
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

      // Move cursor to inside bold block
      let boldBuffer = "";
      for (cursor += 2; line[cursor] + line[cursor + 1] !== "**"; cursor++) {
        boldBuffer += line[cursor];
      }

      if (!boldBuffer) {
        contents.push({ type: "text", value: "****" });
      } else {
        contents.push({
          type: "element",
          tagName: "strong",
          children: inlineFormatting(boldBuffer),
        });
      }

      cursor++;
      continue;
    }

    // Italics (_)
    if (line[cursor] === "_" && line.slice(cursor + 1).includes("_")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor to inside italic block
      let italicBuffer = "";
      for (cursor++; line[cursor] !== "_"; cursor++) {
        italicBuffer += line[cursor];
      }

      if (!italicBuffer) {
        contents.push({ type: "text", value: "__" });
      } else {
        contents.push({
          type: "element",
          tagName: "em",
          children: inlineFormatting(italicBuffer),
        });
      }

      continue;
    }

    // Code (`)
    if (line[cursor] === "`" && line.slice(cursor + 1).includes("`")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor to inside code block
      let codeBuffer = "";
      for (cursor++; line[cursor] !== "`"; cursor++) {
        codeBuffer += line[cursor];
      }

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

      // Move cursor to inside bold block
      let strikethroughBuffer = "";
      for (cursor += 2; line[cursor] + line[cursor + 1] !== "~~"; cursor++) {
        strikethroughBuffer += line[cursor];
      }

      if (!strikethroughBuffer) {
        contents.push({ type: "text", value: "~~~~" });
      } else {
        contents.push({
          type: "element",
          tagName: "s",
          children: inlineFormatting(strikethroughBuffer),
        });
      }

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
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor to inside bold block
      let highlightBuffer = "";
      for (cursor += 2; line[cursor] + line[cursor + 1] !== "=="; cursor++) {
        highlightBuffer += line[cursor];
      }

      if (!highlightBuffer) {
        contents.push({ type: "text", value: "====" });
      } else {
        contents.push({
          type: "element",
          tagName: "mark",
          children: inlineFormatting(highlightBuffer),
        });
      }

      cursor++;
      continue;
    }

    // Subscript (~)
    if (line[cursor] === "~" && line.slice(cursor + 1).includes("~")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor to inside subscript block
      let subscriptBuffer = "";
      for (cursor++; line[cursor] !== "~"; cursor++) {
        subscriptBuffer += line[cursor];
      }

      if (!subscriptBuffer) {
        contents.push({ type: "text", value: "~~" });
      } else {
        contents.push({
          type: "element",
          tagName: "sub",
          children: inlineFormatting(subscriptBuffer),
        });
      }

      continue;
    }

    // Superscript (^)
    if (line[cursor] === "^" && line.slice(cursor + 1).includes("^")) {
      // Push existing buffer and reset buffer
      if (buffer) {
        contents.push({ type: "text", value: buffer });
        buffer = "";
      }

      // Move cursor to inside superscript block
      let superscriptBuffer = "";
      for (cursor++; line[cursor] !== "^"; cursor++) {
        superscriptBuffer += line[cursor];
      }

      if (!superscriptBuffer) {
        contents.push({ type: "text", value: "^^" });
      } else {
        contents.push({
          type: "element",
          tagName: "sup",
          children: inlineFormatting(superscriptBuffer),
        });
      }

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
          properties: { href: linkURL },
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

      let math = "";
      for (cursor += 2; line[cursor] + line[cursor + 1] !== "$$"; cursor++) {
        math += line[cursor];
      }

      if (!math) {
        contents.push({ type: "text", value: "$$$$" });
      } else {
        contents.push(renderMath(math, false));
      }

      cursor++;
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
