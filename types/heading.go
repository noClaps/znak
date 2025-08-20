package types

// A heading. This was adapted from [Astro]'s MarkdownHeading type.
//
// [Astro]: https://astro.build
type Heading struct {
	// The depth of the heading element. A h1 would have a depth of 1, for example.
	Depth uint
	// The slug, or ID, of the heading element.
	Slug string
	// The content of the heading element.
	Title string
}
