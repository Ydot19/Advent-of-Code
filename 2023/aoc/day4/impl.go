package day4

import datastructure "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/data_structure"

func CalculateNumberOfScratchCards(cards []*Card) int {
	tracker := make(map[int]*datastructure.Set[int], 0)
	for index, card := range cards {
		scratches := card.NumberOfScratchCards()
		copies := datastructure.NewSet[int]()
		for i := index + 1; i <= index+scratches; i++ {
			copies.Insert(i)
		}
		tracker[index] = copies
	}

	total := 0
	idx := 0
	for idx < len(tracker) {
		total++
		if copies, ok := tracker[idx]; ok {
			queue := datastructure.NewDeque[int]()
			queue.ExtendBack(copies.AsSlice())
			for queue.Len() > 0 {
				if popped := queue.PopFront(); popped != nil {
					total++
					if poppedCopies, ok := tracker[*popped]; ok {
						queue.ExtendBack(poppedCopies.AsSlice())
					}
				}
			}
		}
		idx++
	}

	return total
}