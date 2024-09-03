// This class is based off of the BananaSlug class in github-slugger
export class Slugger {
  #occurrences: Record<string, number>;
  headings: Heading[];

  constructor() {
    this.#occurrences = {};
    this.headings = [];
  }

  slug(heading: string, depth: number) {
    let slug = heading.replaceAll(/[^a-zA-Z0-9]/g, "-").toLowerCase();
    if (this.#occurrences[slug]) {
      slug += `-${this.#occurrences[slug]}`;
      this.#occurrences[slug]++;
    }

    this.headings.push({ depth, slug, title: heading });
    return slug;
  }
}
