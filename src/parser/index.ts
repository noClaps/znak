import type { CodeTheme } from "../../index.ts";
import { renderMath } from "../utils/math.ts";
import { Slugger } from "../utils/slugger.ts";
import { codeBlock } from "./code-block.ts";
import { containers } from "./containers.ts";
import { images } from "./images.ts";
import { inlineFormatting } from "./inline-formatting.ts";
import { orderedListItems, unorderedListItems } from "./list-items.ts";
import { tables } from "./tables.ts";

export function parseHeadings(input: string) {
  const slugger = new Slugger();

  const matches = input.matchAll(/^(#{1,6}) (.+)/gm);
  for (const match of matches) {
    const level = match[1].length;
    const heading = match[2];
    slugger.slug(heading, level);
  }

  return slugger.headings;
}

export async function parse(input: string, codeTheme: CodeTheme) {
  const slugger = new Slugger();
  const lines = input.trim().split("\n");
  const tokens: (HastElement | HastText)[] = [];
  let buffer = "";

  for (let lineCursor = 0; lineCursor < lines.length; lineCursor++) {
    // Headings
    if (lines[lineCursor].match(/^#{1,6} .+/gm)) {
      const match = lines[lineCursor].match(/^(#{1,6}) (.+)/m)!;
      const level = match[1].length;
      const content = match[2];
      const slug = slugger.slug(content, level);
      tokens.push({
        type: "element",
        tagName: `h${level}`,
        children: inlineFormatting(content),
        properties: { id: slug },
      });
      continue;
    }

    // Blockquotes
    if (lines[lineCursor].startsWith(">")) {
      while (lines[lineCursor] && lines[lineCursor].startsWith(">")) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }
      const blockquoteLines = buffer
        .split("\n")
        .map((line) => line.slice(1).trim())
        .join("\n");

      tokens.push({
        type: "element",
        tagName: "blockquote",
        children: await parse(blockquoteLines, codeTheme),
      });
      buffer = "";
      continue;
    }

    // Horizontal rule
    if (lines[lineCursor].match(/^-{3,}$/m)) {
      tokens.push({ type: "element", tagName: "hr", children: [] });
      buffer = "";
      continue;
    }

    // Images
    if (lines[lineCursor].startsWith("![") && lines[lineCursor].endsWith(")")) {
      tokens.push(images(lines[lineCursor]));
      continue;
    }

    // Code block
    if (
      lines[lineCursor].startsWith("```") &&
      lines
        .slice(lineCursor + 1)
        .find((l) =>
          l.startsWith(
            "`".repeat((lines[lineCursor].match(/`/g) || []).length),
          ),
        )
        ?.endsWith("`")
    ) {
      const backtickCount = (lines[lineCursor].match(/`/g) || []).length;

      // Move inside code block
      buffer += `${lines[lineCursor]}\n`;
      lineCursor++;
      while (!lines[lineCursor].endsWith("`".repeat(backtickCount))) {
        buffer += `${lines[lineCursor]}\n`;
        lineCursor++;
      }

      tokens.push(await codeBlock(buffer, codeTheme));
      buffer = "";
      continue;
    }

    // Ordered list (1., 3 space indentation)
    if (lines[lineCursor].match(/^\d+\. /m)) {
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
        children: await orderedListItems(buffer, codeTheme),
      });
      buffer = "";
      continue;
    }

    // Unordered list [-, 2 space indentation]
    if (lines[lineCursor].startsWith("- ")) {
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
        children: await unorderedListItems(buffer, codeTheme),
      });
      buffer = "";
      continue;
    }

    // Tables
    if (lines[lineCursor].startsWith("| ")) {
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
        .find((l) =>
          l.startsWith(
            ":".repeat(
              (lines[lineCursor].split(" ")[0].match(/:/g) || []).length,
            ),
          ),
        )
        ?.endsWith(":")
    ) {
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

      tokens.push(await containers(buffer, codeTheme));
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

  return tokens;
}
