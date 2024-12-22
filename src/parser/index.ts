import { bundledLanguages, type BundledLanguage } from "@noclaps/highlight";
import type { CodeTheme } from "../../index.ts";
import { renderMath } from "../utils/math.ts";
import { Slugger } from "../utils/slugger.ts";
import { highlightSyntax } from "../utils/syntax-highlighting.ts";
import { containers } from "./containers.ts";
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

export function parse(input: string, codeTheme?: CodeTheme) {
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
        properties: new Map([["id", `${slug}`]]),
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
        children: parse(blockquoteLines, codeTheme),
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
      const line = lines[lineCursor];
      const imageSplit = line.lastIndexOf("](");
      const imageTitle = line.slice(2, imageSplit);
      const imageURL = line.slice(imageSplit + 2, -1);

      tokens.push({
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
      });
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
      const codeBuffer: string[] = [];

      // Move inside code block
      const language = lines[lineCursor].replaceAll("`", "");
      for (
        lineCursor++;
        !lines[lineCursor].endsWith("`".repeat(backtickCount));
        lineCursor++
      ) {
        codeBuffer.push(lines[lineCursor]);
      }
      const code = codeBuffer.join("\n").trim();

      if (language) {
        if (bundledLanguages.includes(language)) {
          tokens.push(
            highlightSyntax(code, codeTheme, language as BundledLanguage),
          );
          continue;
        }

        console.error(
          `Language not supported by Shiki: ${language}, continuing as plaintext`,
        );
      }

      tokens.push(highlightSyntax(code, codeTheme));
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
        children: orderedListItems(buffer, codeTheme),
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
        children: unorderedListItems(buffer, codeTheme),
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

      if (!lines.slice(lineCursor - 1).find((line) => line.includes("</"))) {
        tokens.push({ type: "text", value: buffer.trim() });
        continue;
      }

      while (!lines[lineCursor].includes("</")) {
        lineCursor++;
        buffer += `${lines[lineCursor]}\n`;
      }

      tokens.push({ type: "text", value: buffer.trim() });
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

      tokens.push(containers(buffer, codeTheme));
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
