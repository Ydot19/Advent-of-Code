package day1

import (
	"bufio"
	"strconv"
)

func calibrationValue(str string) int {
	n := len(str)
	var initial string
	runes := []rune(str)
	trie := NewTrieNode()
	wordSoFar := ""
	for i := 0; i < n; i++ {
		wordSoFar += string(runes[i])
		trie.Insert(reverseString(wordSoFar))
		if num, ok := foundWordNumber(trie, wordToNumbers(true)); ok {
			initial = num 
			break
		}
		if _, err := strconv.Atoi(string(runes[i])); err == nil {
			initial = string(runes[i])
			break
		}
	}
    

	var final string
	wordSoFar = ""
	trie.Clear()
	for i := n - 1; i >= 0; i-- {
		wordSoFar += string(runes[i])
		trie.Insert(reverseString(wordSoFar))
		if num, ok := foundWordNumber(trie, wordToNumbers(false)); ok {
			final = num
			break
		}
		if _, err := strconv.Atoi(string(runes[i])); err == nil {
			final = string(runes[i])
			break
		}
	}

	sum, _ := strconv.Atoi(initial + final)
	return sum
}

func SumOfCalibrationValues(scanner *bufio.Scanner) int {
	sum := 0
	for scanner.Scan() {
		line := scanner.Text()
		sum += calibrationValue(line)
	}

	return sum
}

func reverseString(str string) string {
	runes := []rune(str)
    for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}
