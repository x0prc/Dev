package main

import "fmt"

ages = map[string]int{
	"John": 37,
	"Doe" : 22,
}

fmt.Println(len(ages))

// insertion
m[key] = elem

// get 
elem = m[key]

// deletion
delete(m, key)

// check existence
elem, ok := m[key] // boolean output

// key types (can use comparable types while declaring types.)
hits := make(map[string]map[string]int)

// returns 0 when no key is present.

// nested maps
func getNameCounts(names []string) map[rune]map[string]int {
	counts := make(map[rune]map[string]int)
	for _, name := range names {
		if name == ""{
			continue
		}
		firstChar := rune(name[0])
		_, ok := counts[firstChar]
		if !ok {
			counts[firstChar] = make(map[string]int)
		}
		counts[firstChar][name]++
	}
	return counts
}
