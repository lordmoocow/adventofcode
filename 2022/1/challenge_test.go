package challenge

import (
	"testing"
)

func TestPart1(*testing.T) {
	c := Challenge{}
	c.Part1()
}

func TestPart2(*testing.T) {
	for i := 0; i < 1000; i++ {
		c := Challenge{}
		c.Part2()
	}
}

func BenchmarkPart1(b *testing.B) {
	c := Challenge{}
	for i := 0; i < b.N; i++ {
		c.Part1()
	}
}

func BenchmarkPart2(b *testing.B) {
	c := Challenge{}
	for i := 0; i < b.N; i++ {
		c.Part2()
	}
}
