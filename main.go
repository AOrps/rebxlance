package main

import (
	"fmt"

	l "github.com/AOrps/rebxlance/src"
)

func main() {

	fmt.Println("Yes")

	tickler := l.GetPrice("AAPL")
	fmt.Println(tickler)
}