package src

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"
)

/*
sectors.go gets the sector of a stock, it is hardcoded rn
*/

type Sector int8

// Enums for all the Sectors
const (
	Domesitc Sector = iota
	Emerging
	Developed
	Government
	TIPS
	REITS
	Individual
)

// Ensure that ENUMS is equal to the amount of Sector Enums
const ENUMS = 7

type SectorFunc interface {
	// Ensure that all the functions are of equal length to (iota) Enums
	String() string
}

// String() -> Enum to String
func (s Sector) String() string {
	switch s {
	case Domesitc:
		return "domestic"
	case Emerging:
		return "emerging"
	case Developed:
		return "developed"
	case Government:
		return "government"
	case TIPS:
		return "tips"
	case REITS:
		return "reits"
	case Individual:
		return "individual"
	}
	return "unknown"
}

// readJsonFile() -> reads the HardCoded JSON file and converts it into a
// map[string][]string i.e a Dictionary that maps a string to a string slice
func readJsonFile() map[string][]string {
	// TODO: Make url
	jsonFile, err := os.Open("src/sectors/sectors.json")

	if err != nil {
		log.Fatal(err)
	}

	defer jsonFile.Close()

	byteValue, _ := ioutil.ReadAll(jsonFile)

	var result map[string][]string
	json.Unmarshal([]byte(byteValue), &result)
	// fmt.Println(result)
	return result
}

func GetSector(stonk string) Sector {

	sectorStonkMap := readJsonFile()

	sectors := make(map[string]Sector)
	var i int8

	for i = 0; i < ENUMS; i++ {
		sector := Sector(i).String()
		stocks := sectorStonkMap[sector]
		for _, stock := range stocks {
			// fmt.Println(stock)
			sectors[stock] = Sector(i)
		}
	}

	// fmt.Println(sectors)
	fmt.Printf("Length of Sectors %d\n", len(sectors))

	// i = 0
	// for sector := range sectors {
	// 	fmt.Print(sector)
	// 	fmt.Print("\t")
	// 	fmt.Println(sectors[sector])
	// 	i++
	// }

	sector, ok := sectors[stonk]

	fmt.Print(ok)

	if ok {
		return sector
	} else {
		return Sector(6) // Individual
	}
}
