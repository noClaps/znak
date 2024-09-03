import type { BundledTheme } from "shiki";
import { renderMath } from "../utils/math.ts";
import blockquotes from "./blockquotes.ts";
import codeBlock from "./code-block.ts";
import containers from "./containers.ts";
import headings from "./headings.ts";
import images from "./images.ts";
import inlineFormatting from "./inline-formatting.ts";
import { orderedListItems, unorderedListItems } from "./list-items.ts";
import tables from "./tables.ts";
import { Slugger } from "../utils/slugger.ts";

export default class Parser {
  #input: string;
  #codeTheme: BundledTheme;
  #slugger: Slugger;

  constructor(input: string, codeTheme: BundledTheme) {
    this.#input = input;
    this.#codeTheme = codeTheme;
    this.#slugger = new Slugger();
  }

  headings() {
    return this.#slugger.headings;
  }

  async parse() {
    const lines = this.#input.trim().split("\n");
    const tokens: (HastElement | HastText)[] = [];
    let buffer = "";

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

        tokens.push(headings(lines[lineCursor], this.#slugger));
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
        tokens.push(await blockquotes(buffer, this.#codeTheme));
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
      if (
        lines[lineCursor].startsWith("![") &&
        lines[lineCursor].endsWith(")")
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

        tokens.push(await codeBlock(buffer, this.#codeTheme));
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
          children: await orderedListItems(buffer, this.#codeTheme),
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
          children: await unorderedListItems(buffer, this.#codeTheme),
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

        tokens.push(await containers(buffer, this.#codeTheme));
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
}
