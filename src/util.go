package src

type Set map[string]struct{}

type SetFunc interface {
	add(string)
	has(string) bool
}

func (s Set) add(stonk string) {
	s[stonk] = struct{}{}
}

func (s Set) has(stonk string) bool {
	_, ok := s[stonk]
	return ok
}

type DictSet map[string]Set

type DictSetFunc interface {
	add(string)
	contains(string)
}

// func (ds DictSet) add(stonk string)
