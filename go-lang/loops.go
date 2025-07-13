package main

import "fmt"

func bulkSend(numMessages int) float64 {
		totalCost := 0.0
		for i := 0; i < numMessages; i++ {      //can omit condition statement, to make it run forever.
			totalCost += 1.0 + (0.01 * float64(i))
		}
		return totalCost
}

// logical ops usage
func fizzbuzz() {
	for i := 1; i <= 100; i++ {
		if i % 3 == 0 && i % 5 == 0 {
			fmt.Println("fizzbuzz")
		} else if i % 3 == 0 {
			fmt.Println("fizz")
		} else if i % 5 == 0 {
			fmt.Println("buzz")      // can use continue for next iteration.
		} else {
			fmt.Println("i")
		}
	}
}
