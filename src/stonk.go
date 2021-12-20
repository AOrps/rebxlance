package src

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
	var price float32
	q, err := quote.Get(stonk)
	if err != nil {
		fmt.Println(err)
	}

	// TODO: Try and Catch or Error Handle the weird nil pointer error when accessing the .RegularMarketPrice
	// Maybe: https://stackoverflow.com/questions/52827318/can-i-get-whether-a-field-has-been-assigned-with-reflection-in-golang

	// Gets 'Regular market Price'
	if !(q.RegularMarketPrice > float64(0)) {
		price = float32(0)
	}

	price = float32(q.RegularMarketPrice)

	return price
}
