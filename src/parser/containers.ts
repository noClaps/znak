import parser from "./index.ts";

export default function containers(input: string): Token {
  const lines = input.split("\n");
  const type = lines[0].split(" ")[1];

  const values = lines[0].split(" ").slice(2).join(" ");
  let valuesCursor = 0;
  let title = "";
  for (
    valuesCursor;
    valuesCursor < values.length && values[valuesCursor] !== "{";
    valuesCursor++
  ) {
    title += values[valuesCursor];
  }

  let attr = "";
  for (
    valuesCursor++;
    valuesCursor < values.length && values[valuesCursor] !== "}";
    valuesCursor++
  ) {
    attr += values[valuesCursor];
  }

  const content = lines.slice(1, -1).join("\n").trim();

  return {
    element: "container",
    contents: parser(content),
    attributes: {
      type,
      title: title.trim(),
      attr,
    },
  };
}
