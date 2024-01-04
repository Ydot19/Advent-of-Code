package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/Ydot19/Advent-of-Code/2023/aoc/day4"
	"github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/utils"
)

var (
	filepath *string
)

func init() {
	filepath = flag.String("filepath", "", "path to file containing values")
}

func main() {
	flag.Parse()

	sc, err := utils.NewScannerFromFilePath(*filepath)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	cards := make([]*day4.Card, 0)
	totalPoints := 0
	for sc.Scan() {
		if txt := sc.Text(); txt != "" {
			card := day4.NewCard(sc.Text())
			cards = append(cards, card)
			totalPoints += card.CalculatePoints()
		}
	}
	// part a
	fmt.Printf("Part One: Points: %d\n", totalPoints)

	// part b
	totalScratchCards := day4.CalculateNumberOfScratchCards(cards)
	fmt.Printf("Part Two: Total Scratch Cards: %d\n", totalScratchCards)
}
