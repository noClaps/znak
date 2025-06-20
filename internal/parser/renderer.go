package parser

import (
	"fmt"
)

func Renderer(token node) string {
	if text, ok := token.(text); ok {
		return text.Value
	}

	element, ok := token.(element)
	if !ok {
		panic("[ERROR] Reached unreachable branch in renderer")
	}

	attributesList := ""
	if element.Properties != nil {
		for key, val := range element.Properties {
			attributesList += fmt.Sprintf(` %s="%s"`, key, val)
		}
	}

	contents := ""
	childrenLen := len(element.Children)

	if childrenLen == 0 {
		return fmt.Sprintf("<%s%s />", element.TagName, attributesList)
	}

	for _, item := range element.Children {
		contents += Renderer(item)
	}
	return fmt.Sprintf("<%s%s>%s</%s>", element.TagName, attributesList, contents, element.TagName)
}
