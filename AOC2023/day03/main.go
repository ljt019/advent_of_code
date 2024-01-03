package main

import (
	"fmt"
	"image"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	dataBytes, _ := ioutil.ReadFile("input.txt")
	dataString := string(dataBytes)

	schematicMap := map[image.Point]rune{}
	for row, line := range strings.Fields(dataString) {
		for col, char := range line {
			if char != '.' && !unicode.IsDigit(char) {
				schematicMap[image.Point{col, row}] = char
			}
		}
	}

	totalPartsSum, totalGearRatios := 0, 0
	partNumbers := map[image.Point][]int{}

	for row, line := range strings.Fields(dataString) {
		for _, match := range regexp.MustCompile(`\d+`).FindAllStringIndex(line, -1) {
			partBounds := map[image.Point]struct{}{}
			for col := match[0]; col < match[1]; col++ {
				for _, delta := range []image.Point{
					{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1},
				} {
					partBounds[image.Point{col, row}.Add(delta)] = struct{}{}
				}
			}

			partValue, _ := strconv.Atoi(line[match[0]:match[1]])
			for point := range partBounds {
				if _, exists := schematicMap[point]; exists {
					partNumbers[point] = append(partNumbers[point], partValue)
					totalPartsSum += partValue
				}
			}
		}
	}

	for point, numbers := range partNumbers {
		if schematicMap[point] == '*' && len(numbers) == 2 {
			totalGearRatios += numbers[0] * numbers[1]
		}
	}

	fmt.Printf("Total sum of part numbers: %d\n", totalPartsSum)
	fmt.Printf("Total sum of gear ratios: %d\n", totalGearRatios)
}
