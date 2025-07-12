package main

import "fmt"

func main() {
		sendsSoFar := 430
		const sendsToAdd = 25
		sendsSoFar = incrementSends(sendsSoFar, sendsToAdd)
		fmt.Println("youve sent", sendsSoFar, "messages")
}

func incrementSends(sendsSoFar, sendsToAdd int) int {
	sendsSoFar = sendsSoFar + sendsToAdd
	return sendsSoFar
}

// ignore y value
func ignore() {
		x, _ := getPoint()
}

// naming return values
func getCoords() (x, y int){
	return
} // automatically returns both x and y. can still do implicit dec.

// early returns
func divide(divided, divisor int) (int, error) {
	if divisor == 0 {
		return 0, errors.New("Cant divide by zero") // error defined early.
	}
	return divided/divisor, nil
}


