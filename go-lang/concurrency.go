package main

import ( 
				"fmt"
				"time"
)

func sendEmail(message string){
	go func() { // concurrent. spawns a goroutine.
		time.Sleep(time.Millisecond * 250)
		fmt.Printf("email received: '%s'\n", message)
	}()
	fmt.Printf("Email sent: '%s'\n", message)
}

// channels
ch := make(chan int)
// sending to a channel
ch <- 76
// receiving from a channel
v := <-ch

// buffered channels
ch := make(chan int, 100) // sending when buffer is full and receiving when buffer is empty.

// closing the buffer
close(ch)
v, ok := <-ch // ok is false and channel is empty.

// selecting through multiple channels
select {
case i, ok := <- chInts: // if ready, exec.
	fmt.println(i)
case s, ok := <- chStrings:
	fmt.println(s) // if all ready, one is chosen randomly.
}
