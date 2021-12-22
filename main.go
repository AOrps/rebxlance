package main

import (
	"bufio"
	"fmt"
	"os"

	src "github.com/AOrps/rebxlance/src"
)

func main() {

	// gui.MAIN()

	/* Gets all the Args, not including the program name*/
	parameters := os.Args[1:]

	if len(parameters) > 0 {
		/* src.ArgForCLI is in cli.go and handles all the CLI stuff */
		src.ArgForCLI(parameters)
	}

	scanner := bufio.NewScanner(os.Stdin)

	var Stonks []src.Stonk

	for {
		scanner.Scan()
		rawText := scanner.Text()
		text := rawText
		// text := strings.Fields(rawText)
		// for _, val := range text {
		// 	fmt.Printf("[%s]", val)
		// }
		if len(text) == 0 {
			break
		}
		price := src.GetMarketPrice(text)
		fmt.Println(price)
		stonk, err := src.GenerateStonk(text)
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
