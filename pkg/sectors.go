package pkg

import (
	"encoding/json"
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
	// TODO: Make url anchored
	jsonFile, err := os.Open("src/sectors/sectors.json")
	if err != nil {
		log.Fatal(err)
	}
	defer jsonFile.Close()
	byteValue, _ := ioutil.ReadAll(jsonFile)
	var result map[string][]string
	json.Unmarshal([]byte(byteValue), &result)
	return result
}

// GetSector() -> gets the sector from the
func GetSector(stonk string) Sector {
	// sectorStonkMap is a raw JSON object meant to parse out each stock from each sector
	sectorStonkMap := readJsonFile()

	sectors := make(map[string]Sector)
	var i int8

	// Iterate through number of ENUMS to get the stocks
	for i = 0; i < ENUMS; i++ {
		sector := Sector(i).String()
		stocks := sectorStonkMap[sector]
		// TODO: Optimize for only one iteration to run O(n) as opposed to the current O(n^2)
		for _, stock := range stocks {
			sectors[stock] = Sector(i)
		}
	}

	// collects sector and checks if value stock is in the hashmap
	sector, ok := sectors[stonk]

	// Checks if Stock is in the Sector Map
	if ok {
		return sector
	} else {
		// Default is Individual
		return Sector(6) // Individual
	}
	// TODO: Error Check for stock being Individual or not a stock (Validity)

}
