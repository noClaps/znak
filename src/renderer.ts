export function renderer(token: HastText | HastElement) {
  switch (token.type) {
    case "text":
      return token.value;

    case "element":
      let attributeList = "";
      if (token.properties) {
        for (const [key, value] of token.properties) {
          attributeList += ` ${key}="${value}"`;
        }
      }

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
