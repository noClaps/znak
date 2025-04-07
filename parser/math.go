package parser

import (
	"strings"

	"github.com/wyatt915/treeblood"
)

// displayMode is `true` for display="block" and `false` for display="inline"
func renderMath(input string, displayMode bool) (node, error) {
	if displayMode {
		val, err := treeblood.DisplayStyle(input, nil)
		if err != nil {
			return nil, err
		}
		val = strings.ReplaceAll(val, ` xmlns="http://www.w3.org/1998/Math/MathML"`, "")
		val = strings.ReplaceAll(val, ` style="font-feature-settings: 'dtls' off;"`, "")
		return text{val}, nil
	}

	val, err := treeblood.InlineStyle(input, nil)
	if err != nil {
		return nil, err
	}
	val = strings.ReplaceAll(val, ` xmlns="http://www.w3.org/1998/Math/MathML"`, "")
	val = strings.ReplaceAll(val, ` style="font-feature-settings: 'dtls' off;"`, "")
	return text{val}, nil
}
