/** A heading. This was taken from Astro's MarkdownHeading type */
export type Heading = {
  /** The depth of the heading element. A h1 would have a depth of 1, for example. */
  depth: number;
  /** The slug, or ID, of the heading element. */
  slug: string;
  /** The content of the heading element. */
  title: string;
};

// This class is based off of the BananaSlug class in github-slugger
export class Slugger {
  #occurrences: Map<string, number>;
  headings: Heading[];

  constructor() {
    this.#occurrences = new Map<string, number>();
    this.headings = [];
  }

  slug(heading: string, depth: number) {
    const cleanHeading = heading.replaceAll(/[^a-zA-Z0-9]/g, "-").toLowerCase();
    let slug = cleanHeading;
    if (this.#occurrences.has(cleanHeading)) {
      slug += `-${this.#occurrences.get(cleanHeading)}`;
      this.#occurrences.set(
        cleanHeading,
        this.#occurrences.get(cleanHeading)! + 1,
      );
    } else {
      this.#occurrences.set(cleanHeading, 1);
    }

    this.headings.push({ depth, slug, title: heading });
    return slug;
  }
}
