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


