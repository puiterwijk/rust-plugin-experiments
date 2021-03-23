package main

import "C"

import (
	"fmt"
)

//export activate
func activate() C.int {
	fmt.Println("Guest Go was activated!")
	return C.int(42)
}

func main() {
}
