package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/AOrps/rebxlance/pkg"
)

const (
	JSONDB = "cmd/db.json"
)

func main() {
	var Stonks []pkg.Stonk
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Print("> ")
		scanner.Scan()
		rawText := scanner.Text()
		text := rawText

		if len(text) == 0 {
			break
		}

		price := pkg.GetMarketPrice(text)
		fmt.Println(price)
		stonk, err := pkg.GenerateStonk(text)

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

}
