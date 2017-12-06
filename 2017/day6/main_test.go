package main

import (
	"fmt"
	"testing"
)

func TestLoadInitialMemory(t *testing.T) {
	filename := "input.txt"
	initialMemory, err := loadInitialMemory(filename)
	if err != nil {
		t.Error("Error loading initial memory")
	}

	expected := [16]int{4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3}
	if initialMemory != expected {
		t.Errorf("Initial memory invalid (%d != %d)", initialMemory, expected)
	}
}

func TestMainAlgorithm(t *testing.T) {
	memory := []int{0, 2, 7, 0}
	fmt.Println("Starting")
	result, err := countReallocations(memory)
	if err != nil {
		t.Error("Error counting reallocations")
	}

	if result != 5 {
		t.Errorf("Invalid answer. Got %d, should be 5", result)
	}
}

func TestEqualityTesting(t *testing.T) {
	/* Lengths different */
	if testEqual([]int{1}, []int{1, 2}) {
		t.Error("Should not be equal")
	}

	if testEqual([]int{1}, []int{1, 2}) {
		t.Error("Should not be equal")
	}

	if !testEqual([]int{1, 2}, []int{1, 2}) {
		t.Error("Should be equal")
	}
}
