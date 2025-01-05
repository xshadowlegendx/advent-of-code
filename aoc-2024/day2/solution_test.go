package day2

import (
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
			name:  "example",
			want:  161,
			input: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, SolutionPart0(tt.input))
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
			name:  "example",
			want:  48,
			input: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, SolutionPart1(tt.input))
		})
	}
}
