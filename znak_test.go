package znak_test

import (
	"maps"
	"slices"
	"testing"

	"github.com/noClaps/znak"
	"github.com/noClaps/znak/types"
)

func TestHeadings(t *testing.T) {
	headings := znak.ParseHeadings("## Heading 2")
	if !slices.Equal(headings, []types.Heading{
		{Depth: 2, Slug: "heading-2", Title: "Heading 2"},
	}) {
		t.Fatal("Parsing headings failed:", headings)
	}
	headings = znak.ParseHeadings("### This_is-a🍪heading")
	if !slices.Equal(headings, []types.Heading{
		{Depth: 3, Slug: "this-is-a-heading", Title: "This_is-a🍪heading"},
	}) {
		t.Fatal("Parsing headings failed:", headings)
	}
}

func TestFrontmatter(t *testing.T) {
	fm, err := znak.ParseFrontmatter(`
---
title: A title
description: Some description here
date: 2025-03-11
---
`)
	if err != nil {
		t.Error(err)
	}
	if !maps.Equal(fm, map[string]string{
		"title":       "A title",
		"description": "Some description here",
		"date":        "2025-03-11",
	}) {
		t.Fatalf("Parsing frontmatter failed: %#v", fm)
	}

	fm, err = znak.ParseFrontmatter(`
---
title: A title
description: Some description here
date: 2025-03-11
---

Some extra text here.
`)
	if err != nil {
		t.Fatal(err)
	}
	if !maps.Equal(fm, map[string]string{
		"title":       "A title",
		"description": "Some description here",
		"date":        "2025-03-11",
	}) {
		t.Fatal("Parsing frontmatter failed:", fm)
	}

	fm, err = znak.ParseFrontmatter(`
---
title: Google: A Misrepresented Evil
---

This was a post about Google. There's also a <hr /> below to see what happens

---
`)
	if err != nil {
		t.Fatal(err)
	}
	if !maps.Equal(fm, map[string]string{"title": "Google: A Misrepresented Evil"}) {
		t.Fatal("Parsing frontmatter failed:", fm)
	}

	fm, err = znak.ParseFrontmatter(`
---
title: Intro to Privacy, Security and Anonymity
description: How to protect yourself from the internet, on the internet
date: 2022-04-06
lastmod: 2023-03-09
---

---

I've really gotten into this stuff over the last 2 years or so. I probably shouldn't have, since I had a lot of (arguably) more important stuff going on during that time, and focusing on that might have been better for me and my future. But I digress.
`)
	if err != nil {
		t.Fatal(err)
	}
	if !maps.Equal(fm, map[string]string{
		"title":       "Intro to Privacy, Security and Anonymity",
		"description": "How to protect yourself from the internet, on the internet",
		"date":        "2022-04-06",
		"lastmod":     "2023-03-09",
	}) {
		t.Fatal("Parsing frontmatter failed:", fm)
	}
}
