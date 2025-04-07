package highlight

import (
	"encoding/json"
)

type Theme struct {
	Color           *string `json:"color,omitempty"`
	BackgroundColor *string `json:"backgroundColor,omitempty"`
	LineNumbers     *struct {
		Color      string `json:"color"`
		RightSpace *uint  `json:"rightSpace,omitempty"`
	} `json:"lineNumbers,omitempty"`
	Highlights *map[string]struct {
		Color           *string `json:"color,omitempty"`
		FontWeight      *uint   `json:"fontWeight,omitempty"`
		FontStyle       *string `json:"fontStyle,omitempty"`
		BackgroundColor *string `json:"backgroundColor,omitempty"`
	} `json:"highlights,omitempty"`
}

func NewTheme(theme string) (Theme, error) {
	t := Theme{}
	err := json.Unmarshal([]byte(theme), &t)
	if err != nil {
		return Theme{}, err
	}
	return t, nil
}
