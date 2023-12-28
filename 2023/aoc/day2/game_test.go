package day2

import (
	"bufio"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCalculateSumOfPossibleGames(t *testing.T) {
	// arrange
	records := []string{
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	}
	strReader := strings.NewReader(strings.Join(records, "\n"))
	sc := bufio.NewScanner(strReader)
	// act
	sum := CalculateSumOfPossibleGames(sc, &GameSet{
		Colors: map[Color]int{
			Red:   12,
			Blue:  14,
			Green: 13,
		},
	})

	// assert
	assert.Equal(t, 8, sum)
}

func TestCalculateSumOfPowerOfSets(t *testing.T) {
	// arrange
	records := []string{
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	}
	strReader := strings.NewReader(strings.Join(records, "\n"))
	sc := bufio.NewScanner(strReader)
	// act
	sum := CalculateSumOfPowerOfSets(sc)

	// assert
	assert.Equal(t, 2286, sum)
}
