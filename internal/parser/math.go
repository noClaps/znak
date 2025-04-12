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
	pitz := treeblood.NewDocument(nil, false)
	if displayMode {
		val, err := pitz.DisplayStyle(input)
		if err != nil {
			return nil, err
		}
		val = strings.ReplaceAll(val, ` class="math-displaystyle"`, "")
		val = minifyMath(val)
		return text{val}, nil
	}

	val, err := pitz.TextStyle(input)
	if err != nil {
		return nil, err
	}

	val = strings.ReplaceAll(val, ` class="math-textstyle"`, "")
	val = minifyMath(val)
	return text{val}, nil
}
