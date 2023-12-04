package day1

import "strconv"

type TrieNode struct {
	Terminal bool
	Childern map[rune]*TrieNode
}

func NewTrieNode() *TrieNode {
	return &TrieNode{
		Childern: make(map[rune]*TrieNode),
	}
}

func (t *TrieNode) Insert(word string) {
	if len(word) == 0 {
		t.Terminal = true
		return
	}

	r := rune(word[0])
	if _, ok := t.Childern[r]; !ok {
		t.Childern[r] = NewTrieNode()
	}

	t.Childern[r].Insert(word[1:])
}

func (t *TrieNode) Clear() {
	t.Terminal = false
	t.Childern = make(map[rune]*TrieNode)
}

func (t *TrieNode) Prefix(word string) bool {
	if len(word) == 0 {
		return true
	}

	r := rune(word[0])
	if _, ok := t.Childern[r]; !ok {
		return false
	}

	return t.Childern[r].Prefix(word[1:])
}

func foundWordNumber(t *TrieNode, prefixTerms map[string]int) (string, bool) {
	for word, num := range prefixTerms {
		if t.Prefix(word) {
			return strconv.Itoa(num), true
		}
	}

	return "0", false
}

func wordToNumbers(reversed bool) map[string]int {
	v := make(map[string]int)
	one := "one"
	two := "two"
	three := "three"
	four := "four"
	five := "five"
	six := "six"
	seven := "seven"
	eight := "eight"
	nine := "nine"
	if reversed {
		one = reverseString(one)
		two = reverseString(two)
		three = reverseString(three)
		four = reverseString(four)
		five = reverseString(five)
		six = reverseString(six)
		seven = reverseString(seven)
		eight = reverseString(eight)
		nine = reverseString(nine)
	}

	v[one] = 1
	v[two] = 2
	v[three] = 3
	v[four] = 4
	v[five] = 5
	v[six] = 6
	v[seven] = 7
	v[eight] = 8
	v[nine] = 9
	return v
}
