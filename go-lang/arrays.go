package main

import "fmt"

func getMessageWithRetries(plan string) ([]string, error)
		allMessages := getMessageWithRetries()
		if plan == planPro {
			return allMessages[:], nil
		}

		if plan == planFree {
			return allMessages[0:2], nil
		}
		return nil, errors.New("unsupported")
}

func (f *File) Read(buf []byte) (n int, err error)

// slices
mySlice := []string{"I", "Am", "A Developer"}
mySlice := make([]int, 5) // and len or cap for length or capacity

slice = append(slice, oneThing) // add values via append

//2D Matrix / Slice of a Slice
rows := [][]int{}

// range 
fruits := []string{"apple", "banana", "grape"}
for i, fruit := range fruits {
	fmt.Println(i, fruit)
}
