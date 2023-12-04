package day1

import (
	"bufio"
	"fmt"
	"os"
)

func NewScanner(fp string) (*bufio.Scanner, error)	 {
	f, err := os.Open(fp)
	if err != nil {
		return nil, fmt.Errorf("failed to open fail: %w", err)
	}

	return bufio.NewScanner(f), nil
}
