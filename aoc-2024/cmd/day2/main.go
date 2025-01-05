package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/xshadowlegendx/advent-of-code/aoc-2024/day2"
)

func main() {
	f, err := os.ReadFile("cmd/day2/puzzle.txt")
	if err != nil {
		fmt.Printf("fail to read puzzle.txt: %s\n", err)
		os.Exit(1)
	}

	input := string(f)
	input = strings.ReplaceAll(input, "\n", "")

	fmt.Printf("solution part0: %d\n", day2.SolutionPart0(input))
	fmt.Printf("solution part1: %d\n", day2.SolutionPart1(input))
}
