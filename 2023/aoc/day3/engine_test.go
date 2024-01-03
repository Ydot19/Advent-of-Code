package day3

import (
	"testing"

	collections "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/collections/slices"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestEngineAddLineGetNumber(t *testing.T) {
	type scenario struct {
		description string
		str         string
		evaluate    func(nums []*Number, err error)
	}

	for _, scenario := range []scenario{
		{
			description: "no numbers",
			str:         "...*......",
			evaluate: func(nums []*Number, err error) {
				require.NoError(t, err)
				require.Empty(t, nums)
			},
		},
		{
			description: "two numbers",
			str:         ".664.598..",
			evaluate: func(nums []*Number, err error) {
				require.NoError(t, err)
				require.Len(t, nums, 2)
				assert.Equal(t, uint(664), nums[0].val)
				assert.Equal(t, uint(598), nums[1].val)
			},
		},
		{
			description: "single edge number",
			str:         ".......58",
			evaluate: func(nums []*Number, err error) {
				require.NoError(t, err)
				require.Len(t, nums, 1)
				assert.Equal(t, uint(58), nums[0].val)
			},
		},
	} {
		t.Run(scenario.description, func(t *testing.T) {
			// arrange
			engine := NewEngine()
			engine.AddLine(scenario.str)

			// act
			resp, err := engine.Numbers()

			// assert
			scenario.evaluate(resp, err)
		})
	}
}

func TestSampleInput(t *testing.T) {
	// arrange
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

	engine := NewEngine()
	for _, str := range sample {
		engine.AddLine(str)
	}

	// act
	nums, err := engine.Numbers()
	require.NoError(t, err)

	validNums := collections.FilterFunc(nums, func(_ int, el *Number) bool {
		return engine.AdjacentToSymbol(el)
	})

	// assert
	res := collections.ReduceFunc(func(acc uint, el *Number) uint {
		return acc + el.val
	})(validNums)

	assert.Equal(t, uint(4361), res)
}

func TestNumberAt(t *testing.T) {
	// arrange
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
		"..91....91",
	}

	engine := NewEngine()
	for _, str := range sample {
		engine.AddLine(str)
	}

	type scenario struct {
		description string
		row         int
		col         int
		evaluate    func(num *uint)
	}

	for _, scenario := range []scenario{
		{
			description: "case a - num (start)",
			row:         0,
			col:         0,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(467), *num)
			},
		},
		{
			description: "case b - num (middle)",
			row:         0,
			col:         1,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(467), *num)
			},
		},
		{
			description: "case c - num (end)",
			row:         0,
			col:         2,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(467), *num)
			},
		},
		{
			description: "case d - not a number char",
			row:         1,
			col:         3,
			evaluate: func(num *uint) {
				require.Empty(t, num)
			},
		},
		{
			description: "case e - not a number char",
			row:         99,
			col:         0,
			evaluate: func(num *uint) {
				require.Empty(t, num)
			},
		},
		{
			description: "case f - not a number char",
			row:         0,
			col:         99,
			evaluate: func(num *uint) {
				require.Empty(t, num)
			},
		},
		{
			description: "case g - num",
			row:         9,
			col:         3,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(664), *num)
			},
		},
		{
			description: "case h - num",
			row:         9,
			col:         6,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(598), *num)
			},
		},
		{
			description: "case i - num (right edge)",
			row:         10,
			col:         9,
			evaluate: func(num *uint) {
				require.NotEmpty(t, num)
				assert.Equal(t, uint(91), *num)
			},
		},
	} {
		t.Run(scenario.description, func(t *testing.T) {
			// act
			val := engine.NumberAt(scenario.row, scenario.col)
			// assert
			scenario.evaluate(val)
		})
	}
}
