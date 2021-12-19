package src

import (
	"fmt"

	"github.com/piquette/finance-go/quote"
)

/*
stonk.go is for segmentation of stock in a sector and getting the Price of the Stonks
*/

type Stonk struct {
	Name   string
	Price  float64
	Sector string
}

func GenerateStonk(name string) (Stonk, error) {
	return Stonk{
		Name:   name,
		Price:  GetMarketPrice(name),
		Sector: GetSector(name),
	}, nil
}

// GetMarketPrice() -> Get's current price of stocks
func GetMarketPrice(stonk string) float64 {
	q, err := quote.Get(stonk)

	if err != nil {
		fmt.Println(err)
	}

	// return float64(0)

	// Gets 'Regular market Price'
	price := q.RegularMarketPrice
	return price
}
