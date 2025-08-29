package highlight

import (
	"encoding/json"
)

type Theme struct {
	Color           string
	BackgroundColor string
	LineNumbers     struct {
		Color      string
		RightSpace uint
	}
	Highlights map[string]struct {
		Color           string
		FontWeight      uint
		FontStyle       string
		BackgroundColor string
	}
}

func NewTheme(theme []byte) (Theme, error) {
	t := Theme{}
	err := json.Unmarshal(theme, &t)
	if err != nil {
		return Theme{}, err
	}
	return t, nil
}
