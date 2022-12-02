package challenge

import (
	"bufio"
	"bytes"
	"os"
	"strconv"
)

type ChallengeParser struct {
	path        string
	file        *os.File
	scanner     *bufio.Scanner
	buffer      []int
}

func dropCR(data []byte) []byte {
	if len(data) > 0 && data[len(data)-1] == '\r' {
		return data[0 : len(data)-1]
	}
	return data
}

func (cp *ChallengeParser) Close() {
	if cp.file != nil {
		cp.file.Close()
	}
}

func (cp *ChallengeParser) Next() []int {
	if cp.scanner == nil {
		cp.file, _ = os.Open(cp.path)
		cp.scanner = bufio.NewScanner(cp.file)
		cp.scanner.Split(func(data []byte, atEOF bool) (advance int, token []byte, err error) {
			if atEOF && len(data) == 0 {
				return 0, nil, nil
			}
			if i := bytes.Index(data, []byte{'\n', '\n'}); i >= 0 {
				return i + 2, dropCR(data[0:i]), nil
			}
			if atEOF {
				return len(data), dropCR(data), nil
			}
			return 0, nil, nil
		})
		cp.buffer = make([]int, 0, 20)
	}

	if cp.scanner.Scan() {
		cp.buffer = cp.buffer[:0]
		lines := cp.scanner.Bytes()
		for {
			if i := bytes.IndexByte(lines, '\n'); i >= 0 {
				j, _ := strconv.Atoi(string(lines[:i]))
				cp.buffer = append(cp.buffer, j)
				lines = lines[i+1:]
			} else {
				i, _ := strconv.Atoi(string(lines))
				cp.buffer = append(cp.buffer, i)
				break
			}
		}
		return cp.buffer
	}

	return nil
}

type Challenge struct{}

func (c *Challenge) Part1() int {
	cp := ChallengeParser{
		path: "/workspaces/advent/2022/1/input",
	}
	defer cp.Close()

	largest := 0
	for {
		elf := cp.Next()
		if elf == nil {
			break
		}

		calories := 0
		for _, c := range elf {
			calories += c
		}

		if calories > largest {
			largest = calories
		}
	}
	return largest
}

func (c *Challenge) Part2() int {
	cp := ChallengeParser{
		path: "/workspaces/advent/2022/1/input",
	}
	defer cp.Close()

	totals := make([]int, 3)
	for {
		elf := cp.Next()
		if elf == nil {
			break
		}

		calories := 0
		for _, c := range elf {
			calories += c
		}

		if calories > totals[2] {
			totals[0], totals[1], totals[2] = totals[1], totals[2], calories
		} else if calories > totals[1] {
			totals[0], totals[1] = totals[1], calories
		} else if calories > totals[0] {
			totals[0] = calories
		}
	}

	return totals[0] + totals[1] + totals[2]
}
