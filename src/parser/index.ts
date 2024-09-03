import GitHubSlugger from "github-slugger";
import blockquotes from "./blockquotes.ts";
import codeBlock from "./code-block.ts";
import containers from "./containers.ts";
import headings from "./headings.ts";
import images from "./images.ts";
import inlineFormatting from "./inline-formatting.ts";
import { orderedListItems, unorderedListItems } from "./list-items.ts";
import tables from "./tables.ts";
import { renderMath } from "../utils/math.ts";

/**
 * A function that parser the input markdown string. The entire markdown string should be passed into this function.
 * @param input The markdown string to be parsed
 * @returns A tree of tokens
 */
export default function parser(
  input: string,
): (HastElement | HastText | HastToken)[] {
  const lines = input.trim().split("\n");
  const tokens: (HastElement | HastText | HastToken)[] = [];
  let buffer = "";
  const slugger = new GitHubSlugger();

  for (let lineCursor = 0; lineCursor < lines.length; lineCursor++) {
    // Headings
    if (lines[lineCursor].match(/^#{1,6} .+/gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      tokens.push(headings(lines[lineCursor], slugger));
      continue;
    }

    // Blockquotes
    if (lines[lineCursor].startsWith(">")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      while (lines[lineCursor] && lines[lineCursor].startsWith(">")) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }
      tokens.push(blockquotes(buffer));
      buffer = "";
      continue;
    }

    // Horizontal rule
    if (lines[lineCursor].match(/^-{3,}$/m)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      tokens.push({ type: "element", tagName: "hr", children: [] });
      buffer = "";
      continue;
    }

    // Images
    if (lines[lineCursor].startsWith("![") && lines[lineCursor].endsWith(")")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      tokens.push(images(lines[lineCursor]));
      continue;
    }

    // Code block
    if (
      lines[lineCursor].startsWith("```") &&
      lines
        .slice(lineCursor + 1)
        .includes("`".repeat((lines[lineCursor].match(/`/g) || []).length))
    ) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      const backtickCount = (lines[lineCursor].match(/`/g) || []).length;

      // Move inside code block
      buffer += `${lines[lineCursor]}\n`;
      lineCursor++;
      while (!lines[lineCursor].endsWith("`".repeat(backtickCount))) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push(codeBlock(buffer));
      buffer = "";
      continue;
    }

    // Ordered list (1., 3 space indentation)
    if (lines[lineCursor].match(/^\d+\. /m)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      while (
        (lines[lineCursor] && lines[lineCursor].match(/^(\d+\. |   |\t)/m)) ||
        lines[lineCursor] === ""
      ) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      // Move back a line to start with the next element
      lineCursor--;

      tokens.push({
        type: "element",
        tagName: "ol",
        children: orderedListItems(buffer),
      });
      buffer = "";
      continue;
    }

    // Unordered list [-, 2 space indentation]
    if (lines[lineCursor].startsWith("- ")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      while (
        (lines[lineCursor] && lines[lineCursor].match(/^(- |  |\t)/m)) ||
        lines[lineCursor] === ""
      ) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      // Move back a line to start with the next element
      lineCursor--;

      tokens.push({
        type: "element",
        tagName: "ul",
        children: unorderedListItems(buffer),
      });
      buffer = "";
      continue;
    }

    // Tables
    if (lines[lineCursor].startsWith("| ")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      while (lines[lineCursor] && lines[lineCursor].startsWith("| ")) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push(tables(buffer));
      buffer = "";
      continue;
    }

    // HTML Elements
    if (lines[lineCursor].startsWith("<")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      buffer += `${lines[lineCursor]}\n`;
      while (!lines[lineCursor].includes("</")) {
        lineCursor++;
        buffer += `${lines[lineCursor]}\n`;
      }

      tokens.push({ type: "text", value: buffer });
      buffer = "";
      continue;
    }

    // Math block
    if (
      lines[lineCursor] === "$$" &&
      lines.slice(lineCursor + 1).includes("$$")
    ) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      for (lineCursor++; lines[lineCursor] !== "$$"; lineCursor++) {
        buffer += lines[lineCursor];
      }

      tokens.push(renderMath(buffer, true));
      buffer = "";
      continue;
    }

    // Container
    if (
      lines[lineCursor].startsWith(":::") &&
      lines
        .slice(lineCursor + 1)
        .includes(
          ":".repeat(
            (lines[lineCursor].split(" ")[0].match(/:/g) || []).length,
          ),
        )
    ) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({
          type: "element",
          tagName: "p",
          children: inlineFormatting(buffer),
        });
        buffer = "";
      }

      const colonCount = (lines[lineCursor].split(" ")[0].match(/:/g) || [])
        .length;

      // Move inside container
      buffer += `${lines[lineCursor]}\n`;
      for (
        lineCursor++;
        lines[lineCursor] !== ":".repeat(colonCount);
        lineCursor++
      ) {
        buffer += `${lines[lineCursor]}\n`;
      }

      tokens.push(containers(buffer));
      buffer = "";
      continue;
    }

    // Paragraph
    for (lineCursor; lines[lineCursor]; lineCursor++) {
      buffer += lines[lineCursor];
    }
    if (buffer) {
      tokens.push({
        type: "element",
        tagName: "p",
        children: inlineFormatting(buffer),
      });
    }
    buffer = "";
  }

  if (buffer) {
    tokens.push({
      type: "element",
      tagName: "p",
      children: inlineFormatting(buffer),
    });
  }

  return tokens;
}
