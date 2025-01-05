package day1

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_Solution(t *testing.T) {
	tests := []struct {
		name               string
		want               int
		input              string
		isTolerateBadLevel bool
	}{
		{
			name:               "example part0",
			want:               2,
			isTolerateBadLevel: false,
			input: `7 6 4 2 1
					1 2 7 8 9
					9 7 6 2 1
					1 3 2 4 5
					8 6 4 4 1
					1 3 6 7 9`,
		},
		{
			name:               "example part1",
			want:               4,
			isTolerateBadLevel: true,
			input: `7 6 4 2 1
					1 2 7 8 9
					9 7 6 2 1
					1 3 2 4 5
					8 6 4 4 1
					1 3 6 7 9`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			input := strings.ReplaceAll(tt.input, "\t", "")

			assert.Equal(t, tt.want, Solution(input, tt.isTolerateBadLevel))
		})
	}
}
