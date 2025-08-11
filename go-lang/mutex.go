package main

import ("fmt"
				"sort"
				"time")

// mutual exclusion
func protected(){
	mux.Lock() 
	defer mux.Unlock()
}
// safe for concurrent read access, not for rw or wr.
func (c *SafeCounter) Inc(key string) {
	c.mu.Lock()
	// Lock so only one goroutine at a time can access the map c.v.
	c.v[key]++
	c.mu.Unlock()
}
