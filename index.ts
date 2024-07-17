import parser from "./src/parser";
import renderer from "./src/renderer";

class Znak {
  #md: string;
  constructor(input: string) {
    this.#md = input;
  }

  async renderToHTML() {
    const parserOutput = parser(this.#md);
    return await Promise.all(
      parserOutput.map(async (po) => await renderer(po)),
    ).then((ro) => ro.join(""));
  }
}

export default Znak;
