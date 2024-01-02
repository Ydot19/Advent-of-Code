package day3

import "unicode"

type CellKind int

const (
	OtherCellKind CellKind = iota
	GearCellKind
	DotCellKind
	NumericCellKind
)

type Cell struct {
	kind CellKind
	val  rune
	row  uint
	col  uint
}

func NewCell(char rune, row, col uint) *Cell {
	var k CellKind
	switch char {
	case '.':
		k = DotCellKind
	case '*':
		k = GearCellKind
	default:
		if unicode.IsDigit(char) {
			k = NumericCellKind
		}
	}

	return &Cell{
		kind: k,
		val:  char,
		row: row,
		col: col,
	}
}

func (c *Cell) IsDigit() bool {
	return c.kind == NumericCellKind
}

func (c *Cell) IsSymbol() bool {
	if c.kind != NumericCellKind && c.kind != DotCellKind {
		return true
	}

	return false
}

func (c *Cell) IsPossibleGear() bool {
	return c.kind == GearCellKind
}
