package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func countChars(runes []rune) map[rune]int {
	m := make(map[rune]int)
	for _, r := range runes {
		elem, ok := m[r]
		if ok {
			// Value was found in map
			m[r] = elem + 1
		} else {
			// Value was not found in map
			m[r] = 1
		}
	}

	return m
}

func checksum(lines []string) {
	nTwice := 0
	nThrice := 0

	for _, line := range lines {
		runes := []rune(line)
		counts := countChars(runes)

		twiceFoundInLine := false
		thriceFoundInLine := false

		for k := range counts {
			count := counts[k]
			if count == 2 {
				if !twiceFoundInLine {
					nTwice++
					twiceFoundInLine = true
				}
			}

			if count == 3 {
				if !thriceFoundInLine {
					nThrice++
					thriceFoundInLine = true
				}
			}
		}
	}

	fmt.Printf("Checksum: %d\n", nTwice*nThrice)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func min3(a, b, c int) int {
	if a < b {
		if a < c {
			return a
		}
		return c
	}

	if b < c {
		return b
	}
	return c
}

func levDist(s string, lenS int, t string, lenT int) int {
	fmt.Printf("Computing lev distance of %s and %s (%d and %d)\n", s, t, lenS, lenT)
	cost := 0

	if lenS == 0 {
		return lenT
	}

	if lenT == 0 {
		return lenS
	}

	if s[lenS-1] == t[lenT-1] {
		cost = 0
	} else {
		cost = 1
	}

	return min3(levDist(s, lenS-1, t, lenT)+1,
		levDist(s, lenS, t, lenT-1)+1,
		levDist(s, lenS-1, t, lenT-1)+cost)
}

func findBoxes(lines []string) {
	a := lines[0]
	b := lines[1]
	fmt.Printf("%s %s %d\n", a, b, levDist(a, len(a), b, len(b)))
}

func main() {
	dat, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}
	str := string(dat)
	lines := strings.Split(str, "\n")

	// checksum(lines)
	findBoxes(lines)
}
