export default function renderer(token: HastText | HastElement) {
  switch (token.type) {
    case "text":
      return token.value;

    case "element":
      const attributeList = token.properties
        ? Object.keys(token.properties)
            .map((key) => ` ${key}="${token.properties![key]}"`)
            .join("")
        : "";

      let contents = "";
      for (const item of token.children) {
        contents += renderer(item);
      }

      if (token.children.length === 0) {
        return `<${token.tagName}${attributeList} />`;
      }

      return `<${token.tagName}${attributeList}>${contents}</${token.tagName}>`;
  }
}
