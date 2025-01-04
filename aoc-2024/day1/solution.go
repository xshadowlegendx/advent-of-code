package day1

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
)

func Solution(input string, isTolerateBadLevel bool) int {
	solution := 0

	var m sync.Mutex
	var wg sync.WaitGroup

	for _, l := range strings.Split(input, "\n") {
		wg.Add(1)

		go func() {
			defer wg.Done()

			n0 := 0
			n1 := 0

			isSafe := true
			isIncreasing := ""

			skips := map[int]int{}

			i := 0
			levels := strings.Split(l, " ")
			for i < len(levels) {
				if skipped, ok := skips[i]; ok {
					if skipped == 0 {
						skips[i] += 1
						i += 1
						continue
					}
				}

				d, err := strconv.Atoi(levels[i])

				if err != nil {
					fmt.Printf("fail to parse int: %s\n", err)
					os.Exit(1)
				}

				if n0 == 0 {
					n0 = d
				} else {
					n1 = d

					inc := ""
					if n1 >= n0 {
						inc = "increasing"
					} else {
						inc = "decreasing"
					}

					if isIncreasing == "" {
						isIncreasing = inc
					}

					isSafe = !(inc != isIncreasing || n0 == n1 || (n0 > n1 && n0-n1 > 3) || (n1 > n0 && n1-n0 > 3))

					n0 = n1
				}

				if !isSafe {
					if isTolerateBadLevel && len(skips) != len(levels) {
						skip := len(skips)
						skipped, ok := skips[skip]
						if !ok {
							skips[skip] = 0
						}

						if skipped == 0 {
							isIncreasing = ""
							isSafe = true
							i = 0
							n0 = 0
							continue
						}
					}

					break
				} else {
					i += 1
				}
			}

			if isSafe {
				m.Lock()
				solution += 1
				m.Unlock()
			}
		}()
	}

	wg.Wait()

	return solution
}
