package day3

import (
	"math"
	"strconv"

	"github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/collections/slices"
	datastructure "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/data_structure"
)

type Engine struct {
	schema [][]*Cell
}

func NewEngine() *Engine {
	return &Engine{
		schema: make([][]*Cell, 0),
	}
}

func (e *Engine) SchemaLen() int {
	return len(e.schema)
}

func (e *Engine) RowLen(row int) int {
	if row < e.SchemaLen() {
		cells := e.schema[row]
		return len(cells)
	}

	return 0
}

func (e *Engine) GetCell(row, col int) *Cell {
	rowLength := e.RowLen(row)
	if col < rowLength {
		return e.schema[row][col]
	}

	return nil
}

func (e *Engine) AddLine(line string) {
	nextRow := make([]*Cell, len(line))
	nextRowNumber := e.SchemaLen()
	for col, char := range line {
		cell := NewCell(char, uint(nextRowNumber), uint(col))
		nextRow[col] = cell
	}

	e.schema = append(e.schema, nextRow)
}

func (e *Engine) PossibleGears() []*Cell {
	var result []*Cell
	for _, cells := range e.schema {
		for _, cell := range cells {
			if cell.IsPossibleGear() {
				result = append(result, cell)
			}
		}
	}

	return result
}

func (e *Engine) Numbers() ([]*Number, error) {
	var nums []*Number
	for row, cells := range e.schema {
		var currNumStr string
		var charIndex uint = 0
		var start uint = math.MaxUint
		var end uint = 0
		for _, cell := range cells {
			if cell.IsDigit() {
				currNumStr += string(cell.val)
				if charIndex < start {
					start = charIndex
				}
				end = charIndex
			} else {
				if currNumStr != "" {
					val, err := strconv.Atoi(currNumStr)
					if err != nil {
						return nil, err
					}
					nums = append(nums, NewNumber(
						uint(val),
						uint(row),
						start,
						end,
					))

					start = math.MaxUint
					end = 0
					currNumStr = ""
				}
			}

			charIndex += 1
		}

		if currNumStr != "" {
			val, err := strconv.Atoi(currNumStr)
			if err != nil {
				return nil, err
			}
			nums = append(nums, NewNumber(
				uint(val),
				uint(row),
				start,
				charIndex-1,
			))
		}
	}

	return nums, nil
}

func (e *Engine) NumberAt(row, col int) *uint {
	rowLength := e.RowLen(row)
	if col < rowLength {
		cell := e.schema[row][col]
		if cell != nil && cell.IsDigit() {
			charQueue := datastructure.NewDeque[rune]()
			charQueue.PushBack(cell.val)
			right := col
		r:
			for right < rowLength {
				right++
				rightCell := e.GetCell(row, right)
				if rightCell != nil {
					if rightCell.IsDigit() {
						charQueue.PushBack(rightCell.val)
					} else {
						break r
					}
				} else {
					break r
				}
			}

			if col > 0 {
				left := col
			l:
				for left > 0 {
					left--
					leftCell := e.GetCell(row, left)
					if leftCell != nil {
						if leftCell.IsDigit() {
							charQueue.PushFront(leftCell.val)
						} else {
							break l
						}
					} else {
						break l
					}
				}
			}

			num, err := strconv.Atoi(collections.ReduceFunc(func(acc string, char rune) string {
				return acc + string(char)
			})(charQueue.AsSlice()))

			if err != nil {
				return nil
			}

			n := uint(num)
			return &n
		}
	}

	return nil
}

func (e *Engine) AdjacentToSymbol(num *Number) bool {
	// left
	left := num.start_index
	if left > 0 {
		left--
		if cell := e.GetCell(int(num.row), int(left)); cell != nil {
			if cell.IsSymbol() {
				return true
			}
		}
	}

	// right
	right := num.end_index + 1
	if cell := e.GetCell(int(num.row), int(right)); cell != nil {
		if cell.IsSymbol() {
			return true
		}
	}

	// above + below
	shouldSearchAbove := num.row > 0
	for i := left; i <= right; i++ {
		if shouldSearchAbove {
			if cell := e.GetCell(int(num.row-1), int(i)); cell != nil {
				if cell.IsSymbol() {
					return true
				}
			}
		}

		if cell := e.GetCell(int(num.row+1), int(i)); cell != nil {
			if cell.IsSymbol() {
				return true
			}
		}
	}

	return false
}
