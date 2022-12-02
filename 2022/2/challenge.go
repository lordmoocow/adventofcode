package challenge

import (
	"bufio"
	"os"
)

type Strategy struct {
	predicted int
	response  int
}

type ChallengeParser struct {
	path        string
	file        *os.File
	scanner     *bufio.Scanner
}

func (cp *ChallengeParser) Close() {
	if cp.file != nil {
		cp.file.Close()
	}
}

func (cp *ChallengeParser) Next() *Strategy {
	if cp.scanner == nil {
		cp.file, _ = os.Open(cp.path)
		cp.scanner = bufio.NewScanner(cp.file)
	}

	if cp.scanner.Scan() {
		line := cp.scanner.Bytes()
		return &Strategy {
			predicted: int(line[0]) - 64, 
			response: int(line[2]) - 87,
		}
	}

	return nil
}

type Challenge struct{}

func (c *Challenge) Part1() int {
	cp := ChallengeParser{
		path: "/workspaces/advent/2022/2/input",
	}
	defer cp.Close()

	score := 0
	for {
		strategy := cp.Next()
		if strategy == nil {
			break
		}

		score += strategy.response

		if strategy.response == strategy.predicted {
			score += 3
		} else if strategy.response == (strategy.predicted % 3) + 1 {
			score += 6
		}
		//fmt.Printf("%v vs %v\n", string(rune(strategy.predicted+64)), string(rune(strategy.response+87)))
		//fmt.Printf("   %v\n", (strategy.predicted % 3) + 1)
	}
	return score
}

func (c *Challenge) Part2() int {
	cp := ChallengeParser{
		path: "/workspaces/advent/2022/2/input",
	}
	defer cp.Close()

	score := 0
	for {
		strategy := cp.Next()
		if strategy == nil {
			break
		}

		switch strategy.response {
		case 1:
			score += ((strategy.predicted + 1) % 3) + 1
		case 2:
			score += 3 + strategy.predicted
		case 3:
			score += 6 + (strategy.predicted % 3) + 1
		}

		// fmt.Printf("%v vs %v\n", string(rune(strategy.predicted+64)), string(rune(strategy.response+87)))
		// fmt.Printf("  to win: %v\n", string(rune(((strategy.predicted) % 3) + 1 + 64)))
		// fmt.Printf("  to lse: %v\n", string(rune(((strategy.predicted+1) % 3) + 1 + 64)))
		// fmt.Printf("  to drw: %v\n", string(rune((strategy.predicted + 64))))
	}

	return score
}
