package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	dat, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}
	str := string(dat)
	lines := strings.Split(str, "\n")

	var value int64
	value = 0
	for _, line := range lines {
		runes := []rune(line)
		intval, err := strconv.ParseInt(string(runes[1:]), 10, 64)
		if err != nil {
			panic(err)
		}

		if runes[0] == '+' {
			value += intval
		} else if runes[0] == '-' {
			value -= intval
		} else {
			err := fmt.Errorf("invalid prefix: %c", runes[0])
			if err != nil {
				panic(err)
			}
		}
	}

	fmt.Printf("Answer: %v\n", value)
}
