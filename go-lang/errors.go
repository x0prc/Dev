package main

import (
	"errors"
	"fmt")

func main(){
	user, err := getUser()
	if err != nil {
		fmt.Println(err)
		return
	}
	profile, err := getUserProfile(user.ID)
	if err != nil {
		fmt.Println(err)
		return
	}
}

// formatting strings 
const name = "Kim"
const age = 22
s := fmt.Sprintf("%v is %v years old", name, age) // str interpolation

// can build your own errs
type userError struct {
	name string
}

func (e userError) Error() string {
	return fmt.Sprintf("%v has a problem", e.name)
}

// errors lib
var err error = errors.New("somes wrong")
