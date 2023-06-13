package main

import (
	//	"bufio"
	"fmt"
	//	"os"
	"github.com/piquette/finance-go/quote"
	"log"

	//	"github.com/AOrps/rebxlance/portfolio"
)

const (
	JSONDB = "cmd/db.json"
)

func main() {
	q, err := quote.Get("AAPL")

	if err != nil {
		fmt.Println("fucking error")
		log.Fatal(err)
	}
	
	fmt.Println(q)
	/*
	var Stonks []portfolio.Stonk


	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Print("> ")
		scanner.Scan()
		rawText := scanner.Text()
		text := rawText

		if len(text) == 0 {
			break
		}

		price := portfolio.GetMarketPrice(text)
		fmt.Println(price)
		stonk, err := portfolio.GenerateStonk(text)

		if err != nil {
			fmt.Println(err)
		} else {
			Stonks = append(Stonks, stonk)
		}
	}

	fmt.Println("===============================================================")
	for _, stonk := range Stonks {
		fmt.Printf("Name:%s\nPrice:%f\nSector:%s\n\n", stonk.Name, stonk.Price, stonk.Sector)
	}
	fmt.Println("===============================================================")
	*/
}
