type HastToken = {
  type: "token";
  tokenName: string;
  properties?: Record<string, string>;
  children: (HastElement | HastText | HastToken)[];
};

type HastElement = {
  type: "element";
  tagName: string;
  properties?: Record<string, string>;
  children: (HastElement | HastText | HastToken)[];
};

type HastText = {
  type: "text";
  value: string;
};

/** A heading. This was taken from Astro's MarkdownHeading type */
type Heading = {
  /** The depth of the heading element. A h1 would have a depth of 1, for example. */
  depth: number;
  /** The slug, or ID, of the heading element. */
  slug: string;
  /** The content of the heading element. */
  title: string;
};
