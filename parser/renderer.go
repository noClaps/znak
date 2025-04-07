package parser

import (
	"fmt"
)

func Renderer(token node) string {
	switch token.(type) {
	case text:
		return token.(text).Value
	case element:
		element := token.(element)

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

	panic("[ERROR] Reached unreachable branch in renderer")
}
