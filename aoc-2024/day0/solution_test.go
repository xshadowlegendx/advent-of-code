package day0

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_SolutionPart0(t *testing.T) {
	tests := []struct {
		name  string
		want  int
		input string
	}{
		{
			name: "example",
			want: 11,
			input: `3   4
					4   3
					2   5
					1   3
					3   9
					3   3`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			input := strings.ReplaceAll(tt.input, "\t", "")

			assert.Equal(t, tt.want, SolutionPart0(input))
		})
	}
}

func Test_SolutionPart1(t *testing.T) {
	tests := []struct {
		name  string
		want  int
		input string
	}{
		{
			name: "example",
			want: 31,
			input: `3   4
					4   3
					2   5
					1   3
					3   9
					3   3`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			input := strings.ReplaceAll(tt.input, "\t", "")

			assert.Equal(t, tt.want, SolutionPart1(input))
		})
	}
}
