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

func (deq *Deque[T]) AsSlice() []T {
	return deq.items
}