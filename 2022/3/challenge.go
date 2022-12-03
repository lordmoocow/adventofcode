package challenge

import (
	"bufio"
	"os"
)

type Rucksack struct {
	compartment1 []rune
	compartment2 []rune
}

type ChallengeParser struct {
	file    *os.File
	scanner *bufio.Scanner
	current Rucksack
}

func (cp *ChallengeParser) Close() {
	if cp.file != nil {
		cp.file.Close()
	}
}

func (cp *ChallengeParser) Next() bool {
	if cp.scanner == nil {
		cp.file, _ = os.Open("/workspaces/advent/2022/3/input")
		cp.scanner = bufio.NewScanner(cp.file)
	}

	if cp.scanner.Scan() {
		line := []rune(cp.scanner.Text())
		cp.current = Rucksack{
			compartment1: line[0:len(line)/2],
			compartment2: line[len(line)/2:],
		}
		return true
	}

	return false
}

type Challenge struct{}



func ToPriority(b rune) int {
	if b >= 97 {
		return int(b - 96)
	} else {
		return int(b - 38)
	}
}

func FindDuplicates(a []rune, b []rune) []rune {
	common := make([]rune, 0, len(a))
	outer: for _, item := range a {
		// have we already found it before?
		for _, c := range common {
			if c == item {
				continue outer
			}
		}

		for _, item2 := range b {
			if item2 == item {
				common = append(common, item)
				continue outer
			}
		}
	}
	return common
}

func (c *Challenge) Part1() int {
	cp := ChallengeParser{}
	defer cp.Close()

	sum := 0
	var rucksack *Rucksack
	for cp.Next() {
		rucksack = &cp.current
		common := FindDuplicates(rucksack.compartment1, rucksack.compartment2)
		for _, c := range common {
				sum += ToPriority(c)
		}
	}
	return sum
}

func (c *Challenge) Part2() int {
	cp := ChallengeParser{}
	defer cp.Close()


	badges := make([]rune, 0, 100)
	group := make([]Rucksack, 0, 3)
	for cp.Next() {
		group = append(group, cp.current)
		if len(group) < 3 {
			continue
		} else if len(group) > 3 {
			group = group[3:]
			continue
		}
		
		// fmt.Printf("%v\n", string(append(group[0].compartment1, group[0].compartment2...)))
		// fmt.Printf("%v\n", string(append(group[1].compartment1, group[1].compartment2...)))
		// fmt.Printf("%v\n", string(append(group[2].compartment1, group[2].compartment2...)))

		badge := FindDuplicates(
			FindDuplicates(
				append(group[0].compartment1, group[0].compartment2...), 
				append(group[1].compartment1, group[1].compartment2...),
			), 
			append(group[2].compartment1, group[2].compartment2...),
		)[0]

		badges = append(badges, badge)
	}

	sum := 0
	for _, b := range badges {
		sum += ToPriority(b)
	}
	return sum
}
