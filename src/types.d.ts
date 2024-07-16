/** An individual token, with the HTML element and the contents. */
type Token = {
  /** The HTML element to be used for this token. */
  element: string;
  /** The contents of this token. Can be a string or a list of tokens or strings. */
  contents: (string | Token)[];
  attributes?: { [key: string]: string };
};
