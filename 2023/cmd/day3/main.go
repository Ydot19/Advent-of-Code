package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/Ydot19/Advent-of-Code/2023/aoc/day3"
	collections "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/collections/slices"
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

	engine := day3.NewEngine()
	for sc.Scan() {
		engine.AddLine(sc.Text())
	}

	// part a
	nums, err := engine.Numbers()
	if err != nil {
		fmt.Printf("failed to get engine numbers: %s", err.Error())
		os.Exit(1)
	}
	validNums := collections.FilterFunc(nums, func(_ int, el *day3.Number) bool {
		return engine.AdjacentToSymbol(el)
	})
	res := collections.ReduceFunc(func(acc uint, el *day3.Number) uint {
		return acc + el.AsUint()
	})(validNums)

	fmt.Printf("Sum of Part Numbers: %d\n", res)

	// part b
	ratio := day3.CalculateGearRatio(engine)

	fmt.Printf("Gear Ratio: %d\n", ratio)
}
