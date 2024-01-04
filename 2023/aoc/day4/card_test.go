package day4

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCardFromString(t *testing.T) {
	// arrange
	str := "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

	// act
	card := NewCard(str)

	// assert
	require.NotEmpty(t, card)
	assert.Equal(t, 8, card.CalculatePoints())
}

func TestSampleCase_TotalPoints(t *testing.T) {
	strSlice := []string{
		"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
		"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
		"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
		"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
		"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
		"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
	}

	cards := make([]*Card, len(strSlice))
	for idx, str := range strSlice {
		cards[idx] = NewCard(str)
	}

	// act
	actual := 0
	for _, c := range cards {
		require.NotEmpty(t, c)
		actual += c.CalculatePoints()
	}

	// assert
	assert.Equal(t, 13, actual)
}
