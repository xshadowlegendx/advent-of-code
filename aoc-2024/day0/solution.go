package day0

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func SolutionPart0(input string) int {
	left := []int{}
	right := []int{}

	input = strings.ReplaceAll(input, "\t", "")

	for _, s := range strings.Split(input, "\n") {
		ss := strings.Split(s, "   ")

		l, err := strconv.Atoi(ss[0])
		if err != nil {
			fmt.Printf("fail to parse left: %s\n", err)
			os.Exit(1)
		}
		left = append(left, l)

		r, err := strconv.Atoi(ss[1])
		if err != nil {
			fmt.Printf("fail to parse right: %s\n", err)
			os.Exit(1)
		}
		right = append(right, r)
	}

	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})
	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})

	solution := 0

	for i := range len(left) {
		l := left[i]
		r := right[i]

		if l > r {
			solution += l - r
		} else {
			solution += r - l
		}
	}

	return solution
}

func SolutionPart1(input string) int {
	left := []int{}
	rm := map[int]int{}

	input = strings.ReplaceAll(input, "\t", "")

	for _, s := range strings.Split(input, "\n") {
		ss := strings.Split(s, "   ")

		l, err := strconv.Atoi(ss[0])
		if err != nil {
			fmt.Printf("fail to parse left: %s\n", err)
			os.Exit(1)
		}
		left = append(left, l)

		r, err := strconv.Atoi(ss[1])
		if err != nil {
			fmt.Printf("fail to parse right: %s\n", err)
			os.Exit(1)
		}
		if _, ok := rm[r]; !ok {
			rm[r] = 0
		}
		rm[r] += 1
	}

	solution := 0
	for _, v := range left {
		if rmv, ok := rm[v]; ok {
			solution += v * rmv
		}
	}

	return solution
}
