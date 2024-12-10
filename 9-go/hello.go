package main

import (
	"fmt"
	"math"
	"os"
)

func isProcessed(blocks []rune) bool {
    isEmpty := false

    for _, char := range blocks {
        if char == '.' {
            isEmpty = true
        } else {
            if isEmpty {
                return false
            }
        }
    }

    return true
}

func process(blocks []rune) []rune {
    firstEmpty := 0
    lastBlock := math.MaxInt

    for i, char := range blocks {
        if char == '.' {
            firstEmpty = i
            break
        }
    }

    for i := range blocks {
        revI := len(blocks) - i - 1
        char := blocks[revI]

        if char != '.' {
            lastBlock = revI
            break
        }
    }

    id := blocks[lastBlock]
    blocks[lastBlock] = '.'
    blocks[firstEmpty] = id

    return blocks
}

func hash(blocks []rune) int {
    sum := 0

    for i, char := range blocks {
        if char == '.' {
            continue
        }

        id := int(char - 48)
        sum += i * id
    }

    return sum
}

func parseInput(input string) []rune {
    blocks := []rune{}

    for index, char := range input {
        num := int(char - 48)

        if index % 2 == 0 {
            for i := 0; i < num; i++ {
                blocks = append(blocks, rune(index / 2 + 48))
            }
        } else {
            for i := 0; i < num; i++ {
                blocks = append(blocks, '.')
            }
        }
    }

    return blocks
}

func main() {
    data, err := os.ReadFile("input.txt")
    if err != nil {
        panic(err)
    }
    input := string(data)
    blocks := parseInput(input)

    for !isProcessed(blocks) {
        blocks = process(blocks)
    }

    fmt.Println(hash(blocks))
}
