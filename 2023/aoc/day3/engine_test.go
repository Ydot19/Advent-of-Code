package day3

import (
	"testing"

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
	
}
