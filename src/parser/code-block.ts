export default function codeBlock(input: string): HastToken {
  const lines = input.split("\n");
  const language = lines[0].replaceAll("`", "");
  const code = lines.slice(1, -1).join("\n").trim();

  return {
    type: "token",
    tokenName: "code-block",
    children: [{ type: "text", value: code }],
    properties: { "data-lang": language },
  };
}
