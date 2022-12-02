package challenge

import (
	"testing"
)

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
