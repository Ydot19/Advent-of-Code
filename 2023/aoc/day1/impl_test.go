package day1

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCalibrationValue(t *testing.T) {
	tests := []struct {
		val      string
		expected int
	}{
		{
			val:      "1abc2",
			expected: 12,
		},
		{
			val:      "pqr3stu8vwx",
			expected: 38,
		},
		{
			val:      "a1b2c3d4e5f",
			expected: 15,
		},
		{
			val:      "treb7uchet",
			expected: 77,
		},
		{
			val: "two1nine",
			expected: 29,
		},
		{
			val: "eightwothree",
			expected: 83,
		},
		{
			val: "abcone2threexyz",
			expected: 13,
		},
		{
			val: "xtwone3four",
			expected: 24,
		},
		{
			val: "4nineeightseven2",
			expected: 42,
		},
		{
			val: "zoneight234",
			expected: 14,
		},
		{
			val: "7pqrstsixteen",
			expected: 76,
		},
	}

	for _, test := range tests {
		actual := calibrationValue(test.val)
		assert.Equal(t, test.expected, actual)
	}
}

func TestSumOfCalibrationValues(t *testing.T) {
	// arrange
	fp := "../../fixtures/day1_sample.txt"
	sc, err := NewScanner(fp)
	require.NoError(t, err)

	// act
	actual := SumOfCalibrationValues(sc)

	// assert
	assert.Equal(t, 142, actual)
}

func TestSumOfCalibrationValues_WithStringNumbers(t *testing.T) {
	// arrange
	fp := "../../fixtures/day1_sample2.txt"
	sc, err := NewScanner(fp)
	require.NoError(t, err)

	// act
	actual := SumOfCalibrationValues(sc)

	// assert
	assert.Equal(t, 281, actual)
}
