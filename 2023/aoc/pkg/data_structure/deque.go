package datastructure

import "github.com/Ydot19/Advent-of-Code/2023/aoc/pkg/collections/slices"

type Deque[T any] struct {
	items []T
}

func NewDeque[T any]() *Deque[T] {
	return &Deque[T]{
		items: make([]T, 0),
	}
}

func (deq *Deque[T]) PushFront(val T) {
	deq.items = collections.Prepend[T](deq.items, val)
}

func (deq *Deque[T]) PushBack(val T) {
	deq.items = append(deq.items, val)
}

func (deq *Deque[T]) PopFront() *T {
	n := deq.Len()
	if n > 0 {
		item :=  deq.items[0]
		if n > 1 {
			deq.items = deq.items[1:]
		} else {
			deq.items = make([]T, 0)
		}

		return &item
	}

	return nil
}

func (deq *Deque[T]) PopBack() *T {
	n := deq.Len()
	if n > 0 {
		item :=  deq.items[n-1]
		if n > 1 {
			deq.items = deq.items[0:n-1]
		} else {
			deq.items = make([]T, 0)
		}

		return &item
	}

	return nil
}

func (deq *Deque[T]) Len() int {
	return len(deq.items)
}

func (deq *Deque[T]) AsSlice() []T {
	return deq.items
}

func (deq *Deque[T]) ExtendFront(slice []T) {
	deq.items = append(slice, deq.items...)
}

func (deq *Deque[T]) ExtendBack(slice []T) {
	deq.items = append(deq.items, slice...)
}