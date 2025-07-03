package testutils

import (
	"reflect"
	"strings"

	"golang.org/x/net/html"
)

func compareHTMLNodes(n1, n2 *html.Node) bool {
	if n1 == nil && n2 == nil {
		return true
	}
	if n1 == nil || n2 == nil {
		return false
	}
	if n1.Type != n2.Type || n1.Data != n2.Data {
		return false
	}

	if n1.Type == html.TextNode {
		text1 := strings.TrimSpace(n1.Data)
		text2 := strings.TrimSpace(n2.Data)
		if text1 != text2 {
			return false
		}
	}
	if !compareAttributes(n1.Attr, n2.Attr) {
		return false
	}
	// Compare children
	return compareChildren(n1, n2)
}

func compareAttributes(attr1, attr2 []html.Attribute) bool {
	if len(attr1) != len(attr2) {
		return false
	}

	// Convert to maps for order-independent comparison
	map1 := make(map[string]string)
	map2 := make(map[string]string)

	for _, a := range attr1 {
		map1[a.Key] = a.Val
	}
	for _, a := range attr2 {
		map2[a.Key] = a.Val
	}

	return reflect.DeepEqual(map1, map2)
}

func compareChildren(n1, n2 *html.Node) bool {
	child1 := n1.FirstChild
	child2 := n2.FirstChild

	for child1 != nil || child2 != nil {
		if !compareHTMLNodes(child1, child2) {
			return false
		}
		if child1 != nil {
			child1 = child1.NextSibling
		}
		if child2 != nil {
			child2 = child2.NextSibling
		}
	}

	return true
}
