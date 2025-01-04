package main

import (
	"fmt"
	"os"

	"github.com/xshadowlegendx/advent-of-code/aoc-2024/day1"
)

func main() {
	f, err := os.ReadFile("cmd/day1/puzzle.txt")
	if err != nil {
		fmt.Printf("fail to read puzzle.txt: %s\n", err)
		os.Exit(1)
	}

	input := string(f)

	fmt.Printf("solution part0: %d\n", day1.Solution(input, false))
	fmt.Printf("solution part1: %d\n", day1.Solution(input, true))
}
