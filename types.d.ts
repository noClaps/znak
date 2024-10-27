type HastElement = {
  type: "element";
  tagName: string;
  properties?: Map<string, string>;
  children: (HastElement | HastText)[];
};

type HastText = {
  type: "text";
  value: string;
};
