package day3

type Number struct {
	val uint
	row uint
	start_index uint
	end_index uint
}

func NewNumber(val, row, start, end uint) *Number {
	return &Number{
		val: val,
		row: row,
		start_index: start,
		end_index: end,
	}
}