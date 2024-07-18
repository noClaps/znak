export default function codeBlock(input: string): Token {
  const lines = input.split("\n");
  const language = lines[0].replaceAll("`", "");
  const code = lines.slice(1, -2).join("\n").trim();

  return {
    element: "pre",
    contents: [
      {
        element: "code",
        contents: [code],
        attributes: { "data-lang": language },
      },
    ],
  };
}
