package day3

import (
	collections "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/collections/slice"
	ds "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/data_structure"
)

type Gear struct {
	nums *ds.Set[uint]
}

func NewGear() *Gear {
	return &Gear{
		nums: ds.NewSet[uint](),
	}
}

func (g *Gear) isValid() bool {
	return g.nums.Len() == 2
}

func (g *Gear) insert(val *uint) {
	if val != nil {
		g.nums.Insert(*val)
	}
}

func (g *Gear) product() uint {
	return collections.ReduceFunc(func(acc uint, element uint) uint {
		if acc != 0 {
			return acc * element
		}
		return element
	})(g.nums.AsSlice())
}

func CalculateGearRatio(engine *Engine) uint {
	gears := make([]*Gear, 0)
	for _, cell := range engine.PossibleGears() {
		gear := NewGear()
		// current row
		getNumberForGearAtRow(engine, gear, int(cell.row), int(cell.col))

		// above
		getNumberForGearAtRow(engine, gear, int(cell.row)-1, int(cell.col))

		// below
		getNumberForGearAtRow(engine, gear, int(cell.row)+1, int(cell.col))

		if gear.isValid() {
			gears = append(gears, gear)
		}
	}

	var ratio uint = 0
	for _, g := range gears {
		ratio += g.product()
	}
	return ratio
}

func getNumberForGearAtRow(engine *Engine, gear *Gear, row, center int) {
	// center
	gear.insert(engine.NumberAt(row, center))

	// left
	if center > 0 {
		gear.insert(engine.NumberAt(row, center - 1))
	}

	// right
	gear.insert(engine.NumberAt(row, center+1))
}