package day4

import (
	"math"
	"strconv"
	"strings"

	datastructure "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/data_structure"
)

type Card struct {
	luckyNumbers *datastructure.Set[int]
	hand         []int
}

func NewCard(s string) *Card {
	split_str := strings.Split(strings.Trim(s, " "), ":")
	if len(split_str) != 2 {
		return nil
	}

	v := strings.Split(split_str[1], "|")
	if len(v) != 2 {
		return nil
	}

	luckNumbers := datastructure.NewSet[int]()
	for _, str := range strings.Split(strings.Trim(v[0], " "), " ") {
		str := strings.TrimSpace(str)
		if str != "" {
			num, err := strconv.Atoi(str)
			if err != nil {
				return nil
			}
			luckNumbers.Insert(num)
		}
	}

	var hand []int
	for _, str := range strings.Split(strings.Trim(v[1], " "), " ") {
		str := strings.TrimSpace(str)
		if str != "" {
			num, err := strconv.Atoi(str)
			if err != nil {
				return nil
			}
			hand = append(hand, num)
		}
	}

	return &Card{
		luckyNumbers: luckNumbers,
		hand: hand,
	}
}

func (c *Card) CalculatePoints() int {
	pow := 0
	const BASE = 2

	for _, num := range c.hand {
		if c.luckyNumbers.Has(num) {
			pow += 1
		}
	}

	res := float64(0)
	if pow > 0 {
		res = math.Pow(BASE, float64(pow-1))
	}
	return int(math.Round(res))
}

func (c *Card) NumberOfScratchCards() int {
	scratchCards := 0
	for _, n := range c.hand {
		if c.luckyNumbers.Has(n) {
			scratchCards += 1
		}
	}

	return scratchCards
}
