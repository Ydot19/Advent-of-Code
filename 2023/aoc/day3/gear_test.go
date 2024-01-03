package day3

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGearSample(t *testing.T) {
	// arrange
	engine := NewEngine()
	sample := []string{
		"467..114..",
		"...*......",
		"..35..633.",
		"......#...",
		"617*......",
		".....+.58.",
		"..592.....",
		"......755.",
		"...$.*....",
		".664.598..",
	}

	for _, str := range sample {
		engine.AddLine(str)
	}

	// act
	actual := CalculateGearRatio(engine)

	// assert
	assert.Equal(t, uint(467835), actual)
}