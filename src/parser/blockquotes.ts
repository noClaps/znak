import parser from "./index";

export default function blockquotes(input: string): Token {
  const lines = input
    .split("\n")
    .map((line) => line.slice(1).trim())
    .join("\n");

  return { element: "blockquote", contents: parser(lines) };
}
