package src

import (
	"log"

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

func GenerateStonk(name string) Stonk {
	return Stonk{
		Name:   name,
		Price:  GetMarketPrice(name),
		Sector: GetSector(name),
	}
}

// GetMarketPrice() -> Get's current price of stocks
func GetMarketPrice(stonk string) float64 {
	q, err := quote.Get(stonk)

	if err != nil {
		log.Fatal(err)
	}

	// Gets 'Regular market Price'
	price := q.RegularMarketPrice
	return price
}
