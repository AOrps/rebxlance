package pkg

import (
	"errors"
	"fmt"

	"github.com/piquette/finance-go/quote"
)

/*
stonk.go is for segmentation of stock in a sector and getting the Price of the Stonks
*/

// Stonk:
type Stonk struct {
	Name   string
	Price  float32
	Sector Sector
}

// GenerateStonk() -> Generate a Stonk from name
func GenerateStonk(name string) (Stonk, error) {
	if GetMarketPrice(name) == 0 {
		return Stonk{}, errors.New("invalid stock")
	}

	return Stonk{
		Name:   name,
		Price:  GetMarketPrice(name),
		Sector: GetSector(name),
	}, nil
}

// GetMarketPrice() -> Get's current price of stocks
func GetMarketPrice(stonk string) float32 {
	q, err := quote.Get(stonk)

	// Catches err (but I don't think piquette-go does a good job in handling errors)
	if err != nil {
		fmt.Println(err)
		return float32(0)
	}

	// if stonk doesn't exist return 0
	if q == nil {
		return float32(0)
	}

	// Gets 'Regular market Price'
	return float32(q.RegularMarketPrice)
}
