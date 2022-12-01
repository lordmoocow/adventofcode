package main

import (
	"flag"
	"fmt"
	"os"

	day1 "github.com/lordmoocow/adventofcode/2022/1"
)

type Challenger interface {
	Part1() (int, error)
	Part2() (int, error)
}

var (
	help bool
	day  int
	test bool
)

func main() {
	parseArgs()

	if day > 0 {
		run(day)
	} else {
		for i := 1; i <= 25; i++ {
			run(i)
		}
	}
}

func parseArgs() {
	flag.IntVar(&day, "day", 0, "challenge day to run; runs all  days so far if not provided")
	flag.BoolVar(&test, "test", false, "use test input")
	flag.BoolVar(&help, "help", false, "display this help message")
	flag.Parse()
	if help {
		flag.Usage()
		os.Exit(0)
	}
}

func run(day int) {
	var challenge Challenger
	switch day {
	case 1:
		challenge = &day1.Challenge{}
	default:
		return
	}

	fmt.Printf("Day %v\r\n", day)

	output, err := challenge.Part1()
	check(err)
	fmt.Printf("  Part 1: %v\r\n", output)

	output, err = challenge.Part2()
	check(err)
	fmt.Printf("  Part 2: %v\r\n", output)
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}