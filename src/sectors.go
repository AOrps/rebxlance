package src

/*
sectors.go gets the sector of a stock, it is hardcoded rn
*/

// Until Protobuf bugs are dealt with, yes they are gonna be hardcoded into the program
// Thanks: https://codetree.dev/golang-implementing-sets/

type sector map[string]struct{}

func (s sector) add(stonk string) {
	// For some reason struct{}{} is needed because map needs to map to something
	// https://levelup.gitconnected.com/memory-allocation-and-performance-in-golang-maps-b267b5ad9217
	s[stonk] = struct{}{}
}

// func (s sector) remove(stonk string) {
// 	delete(s, stonk)
// }

func (s sector) has(stonk string) bool {
	_, ok := s[stonk]
	return ok
}

func GetSector(stonk string) string {
	domestic := sector{}

	domestic.add("SPY")
	domestic.add("ITOT")
	domestic.add("VTI")
	domestic.add("VTSAX")
	domestic.add("VOO")
	domestic.add("VII")
	domestic.add("QQQ")
	domestic.add("IWF")

	emerging := sector{}

	emerging.add("SCHE")
	emerging.add("ERUS")
	emerging.add("RSX")
	emerging.add("RSXJ")
	emerging.add("VWO")

	developed := sector{}

	developed.add("VEA")
	developed.add("SCHP")
	developed.add("SPDW")
	developed.add("GOEA")

	government := sector{}

	government.add("EDV")
	government.add("TLT")
	government.add("ZROZ")
	government.add("VGIT")

	tips := sector{}

	tips.add("TIPX")
	tips.add("SCHP")
	tips.add("SPIP")
	tips.add("VTIP")

	reits := sector{}

	reits.add("SAFE")
	reits.add("EPRT")
	reits.add("GLPI")
	reits.add("DLR")
	reits.add("VNQ")

	// domestic, emerging, developed, government, tips, reits

	switch {
	case domestic.has(stonk):
		return "domestic"
	case emerging.has(stonk):
		return "emerging"
	case developed.has(stonk):
		return "developed"
	case government.has(stonk):
		return "government"
	case tips.has(stonk):
		return "tips"
	case reits.has(stonk):
		return "reits"
	default:
		return "individual"
	}

}
