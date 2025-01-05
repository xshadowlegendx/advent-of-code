package day2

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func SolutionPart0(input string) int {
	re, err := regexp.Compile(`mul\((\d+),(\d+)\)`)
	if err != nil {
		fmt.Printf("failed to parse regex: %s", err)
		os.Exit(1)
	}

	solution := 0

	for _, s := range re.FindAllStringSubmatch(input, -1) {
		a, err := strconv.Atoi(s[1])
		if err != nil {
			fmt.Printf("fail to parse s[1]: %s", err)
			os.Exit(1)
		}

		b, err := strconv.Atoi(s[2])
		if err != nil {
			fmt.Printf("fail to parse s[2]: %s", err)
			os.Exit(1)
		}

		solution += a * b
	}

	return solution
}

func SolutionPart1(input string) int {
	// `(^|(?<=do\(\))).*?((?=don't\(\))|$)` - re2 does not support lookahead, lookbehind etc
	re, err := regexp.Compile(`(^|do\(\)).*?(don't\(\)|$)`)
	if err != nil {
		fmt.Printf("failed to parse regex: %s", err)
		os.Exit(1)
	}

	solution := 0

	for _, s := range re.FindAllString(input, -1) {
		solution += SolutionPart0(s)
	}

	return solution
}
