import parser from "./src/parser";
import test from "./test/test.md" with { type: "text" };

const parserOutput = parser(test);
Bun.write("./test/syntax-tree.json", JSON.stringify(parserOutput));
