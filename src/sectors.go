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

// Ensure that ENUMS is i+1 greater than Sector Enums 
const ENUMS = 7


type SectorFunc interface {
	// Ensure that all the functions are of equal length to (iota) Enums
	String() string
	StringSlice() string
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

// StringSlice() -> Enum to String Slice
// func (s Sector) StringSlice() string {
	// return [...]string{"domesitc", "emerging", "developed", "government", "tips", "reits", "individual"}[s]
// }

func readJsonFile(sector string) {
	jsonFile, err := os.Open("sectors/sectors.json")

	if err != nil {
		log.Fatal(err)
	}

	defer jsonFile.Close()

	byteValue, _ := ioutil.ReadAll(jsonFile)

	var result map[string]interface{}
	json.Unmarshal([]byte(byteValue), &result)
	fmt.Println(result[sector])
}

func GetSector(stonk string) string {
	i := const ( iota )
}
