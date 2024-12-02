package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func lineToRuneSlice(line string) []rune {
	return []rune(line)
}

func parseRuneRight(runeSlice []rune) string {
	var finalStringInt string
	var finalStringString string
	var firstNumberFound string

	for _, runeValue := range runeSlice {
		if unicode.IsNumber(runeValue) {
			finalStringInt += string(runeValue)
			if firstNumberFound == "" {
				firstNumberFound = finalStringInt
			}
		} else {
			finalStringString += string(runeValue)
			if strings.Contains(finalStringString, "one") && firstNumberFound == "" {
				firstNumberFound = "1"
			} else if strings.Contains(finalStringString, "two") && firstNumberFound == "" {
				firstNumberFound = "2"
			} else if strings.Contains(finalStringString, "three") && firstNumberFound == "" {
				firstNumberFound = "3"
			} else if strings.Contains(finalStringString, "four") && firstNumberFound == "" {
				firstNumberFound = "4"
			} else if strings.Contains(finalStringString, "five") && firstNumberFound == "" {
				firstNumberFound = "5"
			} else if strings.Contains(finalStringString, "six") && firstNumberFound == "" {
				firstNumberFound = "6"
			} else if strings.Contains(finalStringString, "seven") && firstNumberFound == "" {
				firstNumberFound = "7"
			} else if strings.Contains(finalStringString, "eight") && firstNumberFound == "" {
				firstNumberFound = "8"
			} else if strings.Contains(finalStringString, "nine") && firstNumberFound == "" {
				firstNumberFound = "9"
			}
		}
		if firstNumberFound != "" {
			break
		}
	}
	return firstNumberFound
}

func parseRuneLeft(runeSlice []rune) string {
	var finalStringInt string
	var finalStringString string
	var firstNumberFound string

	for i := len(runeSlice) - 1; i >= 0; i-- {
		if unicode.IsNumber(runeSlice[i]) {
			finalStringInt = string(runeSlice[i]) + finalStringInt
			if firstNumberFound == "" {
				firstNumberFound = finalStringInt
			}
		} else {
			finalStringString = string(runeSlice[i]) + finalStringString
			if strings.Contains(finalStringString, "one") && firstNumberFound == "" {
				firstNumberFound = "1"
			} else if strings.Contains(finalStringString, "two") && firstNumberFound == "" {
				firstNumberFound = "2"
			} else if strings.Contains(finalStringString, "three") && firstNumberFound == "" {
				firstNumberFound = "3"
			} else if strings.Contains(finalStringString, "four") && firstNumberFound == "" {
				firstNumberFound = "4"
			} else if strings.Contains(finalStringString, "five") && firstNumberFound == "" {
				firstNumberFound = "5"
			} else if strings.Contains(finalStringString, "six") && firstNumberFound == "" {
				firstNumberFound = "6"
			} else if strings.Contains(finalStringString, "seven") && firstNumberFound == "" {
				firstNumberFound = "7"
			} else if strings.Contains(finalStringString, "eight") && firstNumberFound == "" {
				firstNumberFound = "8"
			} else if strings.Contains(finalStringString, "nine") && firstNumberFound == "" {
				firstNumberFound = "9"
			}
		}
		if firstNumberFound != "" {
			break
		}
	}
	return firstNumberFound
}

func parseLine(line string) int {
	// Turn the line into a rune slice
	runeSlice := lineToRuneSlice(line)
	// Parse the rune slice from the first rune to the last rune
	int1 := parseRuneRight(runeSlice)
	// Parse the rune slice from the last rune to the first rune
	int2 := parseRuneLeft(runeSlice)
	// Concatenate the strings together
	finalInt, _ := strconv.Atoi(int1 + int2)

	return finalInt
}

func main() {
	fileContents, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fileContentsAsString := string(fileContents)
	lines := strings.Split(fileContentsAsString, "\n")

	var total int
	for _, line := range lines {
		// Parse the line and add the result to the total
		total += parseLine(line)
	}

	fmt.Println(total)
}
