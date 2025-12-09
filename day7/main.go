package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

func main() {
	content, err := os.ReadFile("input7.txt")
	if err != nil {
		panic(err)
	}
	builder := strings.Builder{}
	_, err = builder.Write(content)
	if err != nil {
		panic(err)
	}
	rows := strings.Split(builder.String(), "\n")

	day1, day2 := 0, 0
	var indices []int
	var timelines = make([]int, len(rows[0]))
	for _, row := range rows {
		if len(row) == 0 {
			break
		}
		var newIndices []int
		var newTimelines = make([]int, len(rows[0]))
		lastAdded := -1
		for i, char := range row {
			if char == 'S' {
				newIndices = append(newIndices, i)
				newTimelines[i] = 1
			}
			// I know this is O(w^2 h), but I don't care
			if char == '^' && slices.Contains(indices, i) {
				if lastAdded < i-1 {
					newIndices = append(newIndices, i-1)
				}
				newTimelines[i-1] += timelines[i]
				newTimelines[i+1] += timelines[i]
				newIndices = append(newIndices, i+1)
				lastAdded = i + 1
				day1 += 1
			}
			if char == '.' && slices.Contains(indices, i) {
				if lastAdded < i {
					newIndices = append(newIndices, i)
				}
				newTimelines[i] += timelines[i]
				lastAdded = i
			}
		}
		indices = newIndices
		timelines = newTimelines
	}

	for _, tl := range timelines {
		day2 += tl
	}
	fmt.Printf("Day 1: %d\n", day1)
	fmt.Printf("Day 2: %d\n", day2)
}
