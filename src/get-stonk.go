package src

import (
	"fmt"

	"github.com/piquette/finance-go"
)


func GetPrice(tickler string) float32 {
	q, err := quote.Get(tickler)
	if err != nil {
	  // Uh-oh.  
	  panic(err)
	}
	
	// Success!
	fmt.Println(q)
	return q
}