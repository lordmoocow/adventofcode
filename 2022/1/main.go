package main

import (
	"fmt"
	"os"
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

func parseInput(input []byte) []int {
	lines := strings.Split(string(input), "\n")
	data := make([]int, len(lines))
	for i, line := range lines {
		data[i], _ = strconv.Atoi(line)
	}
	return data
}

func part1(data []int) int {
	return 0
}

func part2(data []int) int {
	return 0
}
