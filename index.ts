import parser from "./src/parser";
import renderer from "./src/renderer";
import test from "./test/test.md" with { type: "text" };

const parserOutput = parser(test);
Bun.write("./test/syntax-tree.json", JSON.stringify(parserOutput));
const renderedHTML = await Promise.all(
  parserOutput.map(async (po) => await renderer(po)),
).then((ro) => ro.join(""));

Bun.write("./test/output.html", renderedHTML);
