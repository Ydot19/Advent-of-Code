package day2

import (
	"bufio"
	"math"
	"regexp"
	"strconv"
	"strings"
)

var regExNumber = regexp.MustCompile(`\d+`)

type Color int

const (
	Red Color = iota
	Blue
	Green
)

func ColorFromString(str string) Color {
	var color Color
	switch strings.ToLower(str) {
	case "red":
		color = Red
	case "blue":
		color = Blue
	case "green":
		color = Green
	}

	return color
}

type GameSet struct {
	Colors map[Color]int
}

type Game struct {
	Number int
	Sets   []GameSet
}

func NewGame(record string) *Game {
	g := &Game{
		Number: 0,
		Sets:   []GameSet{},
	}

	g.Parse(record)
	return g
}

// example record: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
func (g *Game) Parse(record string) {
	sets := []GameSet{}

	// get game number and set as g.Number
	recordStrs := strings.Split(record, ":")
	g.Number, _ = strconv.Atoi(regExNumber.FindString(recordStrs[0]))
	// split record into sets
	setsStr := strings.Split(recordStrs[1], ";")
	for _, setStr := range setsStr {
		set := GameSet{
			Colors: make(map[Color]int),
		}

		// split set into colors
		colorsStr := strings.Split(setStr, ",")
		for _, colorStr := range colorsStr {
			colorStr = strings.TrimSpace(colorStr)

			// split color into count and color
			colorStrs := strings.Split(colorStr, " ")
			count, _ := strconv.Atoi(colorStrs[0])
			color := colorStrs[1]
			set.Colors[ColorFromString(color)] = count
		}

		sets = append(sets, set)
	}

	g.Sets = sets
}

func (g *Game) IsPossible(availableCubes *GameSet) bool {
	for _, set := range g.Sets {
		if set.Colors[Red] > availableCubes.Colors[Red] ||
			set.Colors[Blue] > availableCubes.Colors[Blue] ||
			set.Colors[Green] > availableCubes.Colors[Green] {
			return false
		}
	}

	return true
}

func (g *Game) maxNumberOfCubesPerColorAcrossSets() *GameSet {
	maximums := &GameSet{
		Colors: make(map[Color]int),
	}

	maximums.Colors[Red] = math.MinInt
	maximums.Colors[Blue] = math.MinInt
	maximums.Colors[Green] = math.MinInt

	for _, set := range g.Sets {
		for color, count := range set.Colors {
			if count > maximums.Colors[color] {
				maximums.Colors[color] = count
			}
		}
	}

	return maximums
}

func CalculateSumOfPowerOfSets(records *bufio.Scanner) int {
	var score int

	for records.Scan() {
		record := records.Text()
		if record == "" {
			continue
		}
		game := NewGame(record)
		minimums := game.maxNumberOfCubesPerColorAcrossSets()
		product := minimums.Colors[Red]* minimums.Colors[Blue]* minimums.Colors[Green]
		score += product
	}
	return score
}

func CalculateSumOfPossibleGames(records *bufio.Scanner, availableCubes *GameSet) int {
	var score int

	for records.Scan() {
		if records.Text() == "" {
			continue
		}
		game := NewGame(records.Text())
		if game.IsPossible(availableCubes) {
			score += game.Number
		}
	}
	return score
}
