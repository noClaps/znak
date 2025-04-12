package parser

import (
	"fmt"
	"strings"

	"github.com/noClaps/znak/highlight"
)

func containers(input string, codeTheme highlight.Theme) (node, error) {
	splitInput := strings.SplitN(input, "\n", 2)
	head := splitInput[0]
	body := strings.TrimSpace(splitInput[1])

	headSections := strings.SplitN(head, " ", 3)
	containerType := headSections[1]
	meta := ""
	if len(headSections) > 2 {
		meta = headSections[2]
	}

	nextAttrIndex := strings.IndexRune(meta, '{')
	attr := ""
	if nextAttrIndex != -1 {
		attr = meta[nextAttrIndex+1 : len(meta)-1]
	}

	title := strings.ToUpper(containerType)
	if nextAttrIndex == -1 && meta != "" {
		title = meta
	}
	if nextAttrIndex != -1 && meta[0:nextAttrIndex] != "" {
		title = meta[0:nextAttrIndex]
	}
	title = strings.TrimSpace(title)

	attrMap := map[string]string{}
	if attr != "" {
		for a := range strings.SplitSeq(attr, " ") {
			kvPair := strings.SplitN(a, "=", 2)
			key := kvPair[0]
			val := kvPair[1]
			attrMap[key] = val[1 : len(val)-1]
		}
	}

	class := ""
	if c, ok := attrMap["class"]; ok {
		class = c
	}
	attrMap["class"] = strings.TrimSpace(fmt.Sprintf("znak-container %s %s", containerType, class))

	href := attrMap["href"]
	delete(attrMap, "href")

	content, err := Parse(body, codeTheme)
	if err != nil {
		return nil, err
	}
	return element{"div", attrMap, append([]node{
		element{"p", map[string]string{"class": fmt.Sprintf("%s-heading", containerType)}, []node{
			element{TagName: "b", Children: []node{
				func() node {
					if href == "" {
						return text{title}
					}
					return element{"a", map[string]string{
						"href":   href,
						"target": "_blank",
						"rel":    "noopener noreferrer",
					}, []node{text{title}}}
				}(),
			}},
		}},
	}, content...)}, nil
}
