package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/Ydot19/Advent-of-Code/2023/aoc/day2"
	"github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/utils"
)

var (
	filepath *string
	green    *int
	blue     *int
	red      *int
)

func init() {
	filepath = flag.String("filepath", "", "path to file containing values")
	green = flag.Int("green", 0, "number of green cubes")
	blue = flag.Int("blue", 0, "number of blue cubes")
	red = flag.Int("red", 0, "number of red cubes")
}

func main() {
	flag.Parse()

	sc, err := utils.NewScannerFromFilePath(*filepath)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	gameSum := day2.CalculateSumOfPossibleGames(sc, &day2.GameSet{
		Colors: map[day2.Color]int{
			day2.Red:   *red,
			day2.Blue:  *blue,
			day2.Green: *green,
		},
	})

	sc, _ = utils.NewScannerFromFilePath(*filepath)
	productSum := day2.CalculateSumOfPowerOfSets(sc)

	fmt.Printf("Sum of Possible Games Answer: %d\n", gameSum)
	fmt.Printf("Sum of Power of Sets Answer: %d\n", productSum)
}
