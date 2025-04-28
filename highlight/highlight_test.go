package highlight_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/noClaps/znak/highlight"
)

func TestHighlight(t *testing.T) {
	testLang := "py"
	themeFile, err := os.ReadFile("../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}

	testInput, err := os.ReadFile(fmt.Sprintf("testdata/test.%s", testLang))
	if err != nil {
		t.Fatal(err)
	}

	_, err = highlight.Highlight(string(testInput), testLang, theme)
	if err != nil {
		t.Fatal(err)
	}
}
