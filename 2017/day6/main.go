package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	initialMemory, err := loadInitialMemory("input.txt")
	if err != nil {
		panic(err)
	}
	result, err := countReallocations(initialMemory[:])
	if err != nil {
		panic(err)
	}

	fmt.Printf("%d iterations\n", result)
}

func loadInitialMemory(filename string) ([16]int, error) {
	dat, err := ioutil.ReadFile(filename)
	var memory [16]int
	if err != nil {
		return memory, err
	}

	str := string(dat)

	parts := strings.Fields(str)
	if len(parts) != len(memory) {
		return memory, fmt.Errorf("Not enough elements in input, got %d should be 16", len(parts))
	}

	for i := 0; i < len(parts); i++ {
		value, err := strconv.ParseInt(parts[i], 10, 16)
		if err != nil {
			return memory, err
		}

		memory[i] = int(value)
	}

	return memory, nil
}

func countReallocations(memory []int) (int, error) {
	previousStates := [][]int{}

	stateSeen := false
	startingState := make([]int, len(memory))
	counter := 0
	for {
		c := make([]int, len(memory))
		copy(c, memory)
		previousStates = append(previousStates, c)

		memory = newState(memory)
		if !stateSeen {
			if checkSeenBefore(previousStates, memory) {
				stateSeen = true
				copy(startingState, memory)
			}
		} else {
			if testEqual(memory, startingState) {
				return counter + 1, nil
			}

			counter++
		}
	}
}

func checkSeenBefore(previousStates [][]int, memory []int) bool {
	found := false
	for i := 0; i < len(previousStates); i++ {
		if testEqual(memory, previousStates[i]) {
			found = true
			break
		}
	}

	return found
}

func newState(memory []int) []int {
	val, idx := maxIndex(memory)
	memory[idx] = 0
	length := len(memory)
	for val > 0 {
		idx = (idx + 1) % length
		memory[idx]++
		val--
	}
	return memory
}

func maxIndex(data []int) (int, int) {
	idx := 0
	maxval := data[0]
	length := len(data)
	for i := 0; i < length; i++ {
		if data[i] > maxval {
			maxval = data[i]
			idx = i
		}
	}

	return maxval, idx
}

func testEqual(a, b []int) bool {
	l := len(a)
	if l != len(b) {
		return false
	}

	for i := 0; i < l; i++ {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}
