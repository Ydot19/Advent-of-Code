package collections

import "golang.org/x/exp/constraints"


type Reducable interface {
	constraints.Integer | constraints.Float | constraints.Complex | string
}

func Prepend[T any](slice []T, elems ...T) []T {
    return append(elems, slice...)
}

func ReduceFunc[T any, V Reducable] (accumulator func(acc V, element T) V) func([]T) V {
	return func(slice []T) V {
		var acculumation V
		for _, el := range slice {
			acculumation = accumulator(acculumation, el)
		}

		return acculumation
	}
}

func ApplyFunc[T any](mapper func(index int, element T) T) func(slice *[]T) {
	return func(slice *[]T) {
		if slice == nil {
			return
		}
	
		dereffedSlice := *slice
		for i := 0; i < len(dereffedSlice); i++ {
			element := mapper(i, dereffedSlice[i])
			dereffedSlice[i] = element
		}
	}
}

func FromSlice[T any, V any](transform func(index int, element T) V) func([]T) []V {
	return func(slice []T) []V {
		var res []V
		for i, element := range slice {
			v := transform(i, element)
			res = append(res, v)
		}

		return res
	}
	
}