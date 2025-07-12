package main

import "fmt"

type messageToSend struct {
	phoneNumber int
	message string
}

// nested structs
type message struct {
	Model string
}

// anon structs
myCar := struct {
	Make string
	Model string
} { // can also nest these
	Make: "Volkswagen"
	Model: "Polo GT TSI"
}

// embedded structs (pseudo inheritance)
type car struct {
	make string
	model string
}

type truck struct {
	car
	bedSize int
}

// methods
type rect struct {
	width int
	height int
}

func (r rect) area() int { // special func
	return r.width * r.height
}

r := rect{
	width: 5,
	height: 10,
}

fmt.Println(r.area())
