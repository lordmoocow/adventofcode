package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input, err := os.ReadFile("./input")
	check(err)
	data := parseInput(input)

	fmt.Printf("Part1: %v\n", part1(data))
	fmt.Printf("Part2: %v\n", part2(data))
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func parseInput(input []byte) [][]int {
	groups := strings.Split(string(input), "\r\n\r\n")
	data := make([][]int, len(groups))
	for i, group := range groups {
		lines := strings.Split(group, "\r\n")
		data[i] = make([]int, len(lines))
		for j, line := range lines {
			data[i][j], _ = strconv.Atoi(line)
		}
	}
	return data
}

func part1(data [][]int) int {
	largest := 0
	for _, calories := range data {
		sum := 0
		for _, c := range calories {
			sum += c
		}

		if sum > largest {
			largest = sum
		}
	}
	return largest
}

func part2(data [][]int) int {
	totals := make([]int, len(data))
	for i, calories := range data {
		sum := 0
		for _, c := range calories {
			sum += c
		}
		totals[i] = sum
	}

	sort.Slice(totals, func(i, j int) bool {
		return totals[i] > totals[j]
	})

	sum := 0
	for _, cals := range totals[:3] {
		sum += cals
	}
	return sum
}
