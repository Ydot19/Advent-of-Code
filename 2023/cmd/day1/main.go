package main

import (
	"flag"
	"fmt"
	"log"

	"github.com/Ydot19/Advent-of-Code/2023/aoc/day1"
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
		log.Fatal(err.Error())
	}

	val := day1.SumOfCalibrationValues(sc)
	fmt.Printf("Answer: %d\n", val)
}
