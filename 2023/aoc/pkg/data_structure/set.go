package datastructure

type Set[T comparable] struct {
	set map[T]bool
}

func NewSet[T comparable]() *Set[T] {
	return &Set[T]{
		set: map[T]bool{},
	}
}

func (s *Set[T]) Has(val T) bool {
	return s.set[val]
} 

func (s *Set[T]) Len() int {
	return len(s.set)
}

func (s *Set[T]) Insert(item T) {
	s.set[item] = true
}

func (s *Set[T]) Delete(item T) bool {
	deleted := false
	if s.Has(item) {
		delete(s.set, item)
		deleted = true
	}

	return deleted
}

func (s *Set[T]) AsSlice() []T {
	res := make([]T, s.Len())
	idx := 0
	for key := range s.set {
		res[idx] = key
		idx++
	}

	return res
}