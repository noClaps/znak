import Temml from "temml";

export function renderMath(input: string, isBlock: boolean): HastText {
  return {
    type: "text",
    value: Temml.renderToString(input, {
      displayMode: isBlock,
    }),
  };
}
