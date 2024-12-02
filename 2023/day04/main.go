package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getNumbers(line string) ([]int, error) {
	numbers := []int{}
	for _, numberStr := range strings.Split(line, " ") {
		if numberStr == "" {
			continue
		}
		number, err := strconv.Atoi(numberStr)
		if err != nil {
			return nil, err
		}
		numbers = append(numbers, number)
	}
	return numbers, nil
}

func isMatching(numbersOne []int, numbersTwo []int) bool {
	for _, numberOne := range numbersOne {
		for _, numberTwo := range numbersTwo {
			if numberOne == numberTwo {
				return true
			}
		}
	}
	return false
}

func parseLine(line string) int {
	// cut the "Card x:" part from the line where x is the card number (1-203)
	idx := strings.Index(line, ": ")
	if idx == -1 {
		return 0
	}

	// Cut the "Card x:" part from the line
	line = line[idx+2:]

	parts := strings.Split(line, "|")

	// get the numbers from the first part
	numbersOne, _ := getNumbers(strings.TrimSpace(parts[0]))

	// get the numbers from the second part
	numbersTwo, _ := getNumbers(strings.TrimSpace(parts[1]))

	var matches int
	var points int
	for _, numberOne := range numbersOne {
		for _, numberTwo := range numbersTwo {
			if numberOne == numberTwo {
				matches++
			}
		}
	}

	if matches == 1 {
		points = 1
	} else if matches > 1 {
		points = 1
		for match := 1; match < matches; match++ {
			points = points * 2
		}
	} else {
		points = 0
	}

	return points
}

func main() {
	fileContents, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fileContentsAsString := string(fileContents)
	lines := strings.Split(fileContentsAsString, "\n")

	var lineNumber int
	var points int
	var pointsTotal int
	for _, line := range lines {
		lineNumber++
		points = parseLine(line)
		fmt.Println("Card ", lineNumber, ": ", points)

		pointsTotal += points
	}
	fmt.Println(pointsTotal)
}
