package parser

type node interface {
	sealed()
}

type element struct {
	TagName    string
	Properties map[string]string
	Children   []node
}

func (e element) sealed() {}

type text struct {
	Value string
}

func (t text) sealed() {}
