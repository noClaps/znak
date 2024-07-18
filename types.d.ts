/** An individual token, with the HTML element and the contents. */
type Token = {
  /** The HTML element to be used for this token. */
  element: string;
  /** The contents of this token. Can be a string or a list of tokens or strings. */
  contents: (string | Token)[];
  attributes?: { [key: string]: string };
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
