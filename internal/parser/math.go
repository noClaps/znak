package parser

import (
	"regexp"
	"strings"

	"github.com/wyatt915/treeblood"
)

func minifyMath(input string) string {
	input = strings.ReplaceAll(input, ` xmlns="http://www.w3.org/1998/Math/MathML"`, "")
	input = strings.ReplaceAll(input, ` style="font-feature-settings: 'dtls' off;"`, "")

	re := regexp.MustCompile(`>\s+<`)
	input = re.ReplaceAllString(input, "><")
	re = regexp.MustCompile(`\s+`)
	input = re.ReplaceAllString(input, " ")
	input = strings.TrimSpace(input)

	return input
}

// displayMode is `true` for display="block" and `false` for display="inline"
func renderMath(input string, displayMode bool) (node, error) {
	if displayMode {
		val, err := treeblood.DisplayStyle(input, nil)
		if err != nil {
			return nil, err
		}
		val = minifyMath(val)
		return text{val}, nil
	}

	val, err := treeblood.InlineStyle(input, nil)
	if err != nil {
		return nil, err
	}

	val = minifyMath(val)
	return text{val}, nil
}
