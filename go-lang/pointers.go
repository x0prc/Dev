package main

import ( "fmt"
				"strings"
)

func removeProfanity(message *string) {
	if message == nil {
		return
	}
	messageVal := *message
	messageVal = strings.replaceAll(messageVal, "damn", "****")
	messageVal = strings.replaceAll(messageVal, "shit", "****")
	messageVal = strings.replaceAll(messageVal, "fuck", "****")
}

// pointer receivers
func (c *circle) grow() {
	c.radius *= 2
}

func main(){
	c := circle{
		x: 1,
		y: 2,
		radius: 4,
	}

	c.grow() // not being directly passed; called a receiver.
	fmt.Println(c.radius)
}
