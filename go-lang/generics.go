package main

import "fmt"

// type parameters
func splitAnySlice[T any](s []T) ([]T, []T) { // doesnt care about the datatype.
																							// anything will be sliced.
	mid := len(s)/2
	return s[:mid], s[mid:]
}
