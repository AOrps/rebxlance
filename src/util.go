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
