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

	pkg.FlagsForCLI()

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

	// gui.MAIN()

	/* Gets all the Args, not including the program name*/
	// parameters := os.Args[1:]

	// if len(parameters) > 0 {
	// 	/* src.ArgForCLI is in cli.go and handles all the CLI stuff */
	// 	src.ArgForCLI(parameters)
	// }

	// scanner := bufio.NewScanner(os.Stdin)

	// var Stonks []src.Stonk

	// for {
	// 	scanner.Scan()
	// 	rawText := scanner.Text()
	// 	text := rawText
	// 	// text := strings.Fields(rawText)
	// 	// for _, val := range text {
	// 	// 	fmt.Printf("[%s]", val)
	// 	// }
	// 	if len(text) == 0 {
	// 		break
	// 	}
	// 	price := src.GetMarketPrice(text)
	// 	fmt.Println(price)
	// 	stonk, err := src.GenerateStonk(text)
	// 	if err != nil {
	// 		fmt.Println(err)
	// 	} else {
	// 		Stonks = append(Stonks, stonk)
	// 	}
	// }

	// fmt.Println("===============================================================")
	// for _, stonk := range Stonks {
	// 	fmt.Printf("Name:%s\nPrice:%f\nSector:%s\n\n", stonk.Name, stonk.Price, stonk.Sector)
	// }
	// fmt.Println("===============================================================")

}
