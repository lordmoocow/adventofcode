package challenge

import (
	"os"
	"sort"
	"strconv"
	"strings"
)

type Challenge struct{ }

func parse() [][]int {
	input, _ := os.ReadFile("./1/input")

	groups := strings.Split(string(input), "\n\n")
	elves := make([][]int, len(groups))
	for i, group := range groups {
		lines := strings.Split(group, "\n")
		elves[i] = make([]int, len(lines))
		for j, line := range lines {
			elves[i][j], _ = strconv.Atoi(line)
		}
	}
	return elves
}

func (c *Challenge) Part1() (int, error) {
	elves := parse()
	largest := 0
	for _, elf := range elves {
		calories := 0
		for _, c := range elf {
			calories += c
		}

		if calories > largest {
			largest = calories
		}
	}
	return largest, nil
}

func (c *Challenge) Part2() (int, error) {
	elves := parse()
	totals := make([]int, len(elves))
	for i, elf := range elves {
		calories := 0
		for _, c := range elf {
			calories += c
		}
		totals[i] = calories
	}

	sort.Slice(totals, func(i, j int) bool {
		return totals[i] > totals[j]
	})

	sum := 0
	for _, elf := range totals[:3] {
		sum += elf
	}
	return sum, nil
}
