import inlineFormatting from "./inline-formatting";
import headings from "./headings";
import blockquotes from "./blockquotes";
import images from "./images";
import codeBlock from "./code-block";
import { orderedListItems, unorderedListItems } from "./list-items";
import tables from "./tables";
import GitHubSlugger from "github-slugger";

/**
 * A function that parser the input markdown string. The entire markdown string should be passed into this function.
 * @param input The markdown string to be parsed
 * @returns A tree of tokens
 */
export default function parser(input: string): Token[] {
  const lines = input.trim().split("\n");
  const tokens: Token[] = [];
  let buffer = "";
  const slugger = new GitHubSlugger();

  for (let lineCursor = 0; lineCursor < lines.length; lineCursor++) {
    // Headings
    if (lines[lineCursor].match(/^#{1,6} .+/gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      tokens.push(headings(lines[lineCursor], slugger));
      continue;
    }

    // Blockquotes
    if (lines[lineCursor].startsWith(">")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
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
    if (lines[lineCursor] === "---") {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      tokens.push({ element: "hr", contents: [] });
      buffer = "";
      continue;
    }

    // Images
    if (lines[lineCursor].match(/^!\[.*\]\(.+\)/gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      tokens.push(images(lines[lineCursor]));
      continue;
    }

    // Code block
    if (lines[lineCursor].startsWith("```")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      // Move inside code block
      buffer += `${lines[lineCursor]}\n`;
      lineCursor++;
      while (lines[lineCursor] && lines[lineCursor] !== "```") {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push(codeBlock(buffer));
      buffer = "";
      continue;
    }

    // Ordered list (1., 3 space indentation)
    if (lines[lineCursor].match(/^\d+\. /gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      while (
        lines[lineCursor] &&
        lines[lineCursor].match(/^(\d+\. |\s{3,}).+/gm)
      ) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push({ element: "ol", contents: orderedListItems(buffer) });
      buffer = "";
      continue;
    }

    // Unordered list [-, 2 space indentation]
    if (lines[lineCursor].match(/^- /gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      while (lines[lineCursor] && lines[lineCursor].match(/^(- |\s{2,}).+/gm)) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push({ element: "ul", contents: unorderedListItems(buffer) });
      buffer = "";
      continue;
    }

    // Tables
    if (lines[lineCursor].startsWith("| ")) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
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
    if (lines[lineCursor].match(/^\<.+\>/gm)) {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      while (
        !lines[lineCursor - 1] ||
        !lines[lineCursor - 1].match(/<\/.+\>$/gm)
      ) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push({ element: "raw", contents: [buffer] });
      buffer = "";
      continue;
    }

    // Math block
    if (lines[lineCursor] === "$$") {
      // Dump buffer as paragraph
      if (buffer) {
        tokens.push({ element: "p", contents: inlineFormatting(buffer) });
        buffer = "";
      }

      for (lineCursor++; lines[lineCursor] !== "$$"; lineCursor++) {
        buffer += lines[lineCursor];
      }

      tokens.push({
        element: "math",
        contents: [buffer],
        attributes: { "data-display": "block" },
      });
      buffer = "";
      continue;
    }

    // Paragraph
    while (lines[lineCursor]) {
      buffer += lines[lineCursor];
      lineCursor++;
    }
    if (buffer) {
      tokens.push({ element: "p", contents: inlineFormatting(buffer) });
    }
    buffer = "";
  }

  if (buffer) {
    tokens.push({ element: "p", contents: inlineFormatting(buffer) });
  }

  return tokens;
}
