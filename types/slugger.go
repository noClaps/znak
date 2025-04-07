package types

import (
	"fmt"
	"regexp"
	"strings"
)

type slugger struct {
	occurrences map[string]uint
	Headings    []Heading
}

func NewSlugger() slugger {
	return slugger{
		occurrences: make(map[string]uint),
		Headings:    []Heading{},
	}
}

func (s *slugger) Slug(heading string, depth uint) string {
	headingRe := regexp.MustCompile("[^a-zA-Z0-9]")
	cleanHeading := string(headingRe.ReplaceAll([]byte(heading), []byte("-")))
	cleanHeading = strings.ToLower(cleanHeading)
	slug := strings.Clone(cleanHeading)

	switch occ, ok := s.occurrences[cleanHeading]; ok {
	case true:
		slug += fmt.Sprintf("-%d", occ)
		s.occurrences[cleanHeading] = occ + 1
	case false:
		s.occurrences[cleanHeading] = 1
	}

	s.Headings = append(s.Headings, Heading{
		Depth: depth,
		Slug:  slug,
		Title: heading,
	})
	return slug
}
