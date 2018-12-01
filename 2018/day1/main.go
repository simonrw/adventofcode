package main

import (
	"fmt"
	"io/ioutil"
	"os"
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

	var seenElements []int64
	counter := 0

	for {
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

			if seenElementsContains(seenElements, value) {
				fmt.Printf("First element seen twice: %d\n", value)
				os.Exit(0)
			}

			seenElements = append(seenElements, value)
		}

		if counter == 0 {
			fmt.Printf("Answer: %v\n", value)
		}

		counter++
	}
}

func seenElementsContains(seenElements []int64, value int64) bool {
	for _, testValue := range seenElements {
		if testValue == value {
			return true
		}
	}

	return false
}
