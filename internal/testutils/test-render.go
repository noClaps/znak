package testutils

import (
	"os"
	"strings"
	"testing"

	"github.com/noClaps/znak"
	"github.com/noClaps/znak/highlight"
	"golang.org/x/net/html"
)

func TestRender(input string, test string, t *testing.T) {
	themeFile, err := os.ReadFile("../../theme.json")
	if err != nil {
		t.Fatal(err)
	}
	theme, err := highlight.NewTheme(themeFile)
	if err != nil {
		t.Fatal(err)
	}
	output, err := znak.Render(input, theme)
	if err != nil {
		t.Fatal(err)
	}
	outputNode, err := html.Parse(strings.NewReader(output))
	if err != nil {
		t.Fatal(err)
	}
	testNode, err := html.Parse(strings.NewReader(test))
	if err != nil {
		t.Fatal(err)
	}
	if !compareHTMLNodes(outputNode, testNode) {
		t.Fatalf("Rendering failed.\nInput: %s\nExpected: %s\nReceived: %s\n", input, test, output)
	}
}
