package main

// https://www.stockdata.org/documentation
import (
	"bufio"
	"os"
	"fmt"
	// "github.com/AOrps/rebxlance/portfolio"
)


const (
	JSONDB = "cmd/db.json"
)

func main() {


	scanner := bufio.NewScanner(os.Stdin)

	fmt.Print("> ")
	scanner.Scan()

	rawText := scanner.Text()
	
	fmt.Printf("[%s]\n", rawText)
	// for {
	// 	fmt.Print("> ")
	// 	scanner.Scan()
	// 	rawText := scanner.Text()
	// 	text := rawText

	// 	if len(text) == 0 {
	// 		break
	// 	}

	// 	// price := portfolio.GetMarketPrice(text)
	// 	// fmt.Println(price)
	// 	// stonk, err := portfolio.GenerateStonk(text)
 
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
