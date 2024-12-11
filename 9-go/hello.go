package main

import (
	"fmt"
	"math"
	"os"
)

func isProcessed(blocks []int) bool {
    isEmpty := false

    for _, int := range blocks {
        if int == -1 {
            isEmpty = true
        } else {
            if isEmpty {
                return false
            }
        }
    }

    return true
}

func process(blocks []int) []int {
    firstEmpty := 0
    lastBlock := math.MaxInt

    for i, id := range blocks {
        if id == -1 {
            firstEmpty = i
            break
        }
    }

    for i := range blocks {
        revI := len(blocks) - i - 1
        id := blocks[revI]

        if id != -1 {
            lastBlock = revI
            break
        }
    }

    id := blocks[lastBlock]
    blocks[lastBlock] = -1
    blocks[firstEmpty] = id

    return blocks
}

func hash(blocks []int) int {
    sum := 0

    for i, id := range blocks {
        if id == -1 {
            continue
        }

        sum += i * id
    }

    return sum
}

func parseInput(input string) ([]int, int) {
    blocks := []int{}
    maxId := 0

    for index, char := range input {
        num := int(char - 48)

        if index % 2 == 0 {
            for i := 0; i < num; i++ {
                id := index / 2
                maxId = id
                blocks = append(blocks, id)
            }
        } else {
            for i := 0; i < num; i++ {
                blocks = append(blocks, -1)
            }
        }
    }

    return blocks, maxId
}

func findFile(blocks []int, targetId int) (start int, count int) {
    start, count = 0, 0

    for i, id := range blocks {
        if id == targetId {
            count++

            if start == 0 {
                start = i
            }
        }
    }

    return
}

func process2(blocks []int, maxId int) []int {
    for targetId := maxId; targetId >= 0; targetId-- {
        fileStart, fileLen := findFile(blocks, targetId)

        freeStart, freeLen := 0, 0

        for i := 0; i < fileStart; i++ {
            id := blocks[i]

            if id == -1 {
                if freeLen == 0 {
                    freeStart = i
                }

                freeLen++
            } else {
                if freeLen >= fileLen {
                    // Can move

                    for j := 0; j < fileLen; j++ {
                        blocks[fileStart + j] = -1
                    }

                    for j := 0; j < fileLen; j++ {
                        blocks[freeStart + j] = targetId
                    }

                    break
                } else {
                    // Move on

                    freeStart, freeLen = 0, 0
                }
            }
        }

        // fmt.Println(string(blocks))
    }

    return blocks
}

func main() {
    data, err := os.ReadFile("input.txt")
    if err != nil {
        panic(err)
    }
    input := string(data)
    blocks, maxId := parseInput(input)
    blocks2 := make([]int, len(blocks))
    copy(blocks2, blocks)

    for !isProcessed(blocks) {
        blocks = process(blocks)
    }

    fmt.Println(hash(blocks))

    blocks2 = process2(blocks2, maxId)

    fmt.Println(blocks2)
    fmt.Println(hash(blocks2))
}
