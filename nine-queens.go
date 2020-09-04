package main

import (
	"fmt"
	"time"
)

const (
	N         = 8
	MAX_PRINT = 1
)

// https://oeis.org/A000170

func main() {
	output := [][N]int8{}
	queen_rows := [N]int8{}
	tick := time.Now().UnixNano()
	nine_queens(0, &queen_rows, &output)
	toc := time.Now().UnixNano()

	max_print := MAX_PRINT
	if max_print > len(output) {
		max_print = len(output)
	}
	for i := 0; i < max_print; i++ {
		fmt.Printf("Solution %v\n", i)
		print_board(output[i])
	}
	fmt.Printf("%v queens %v solutions in %d μs\n", N, len(output), (toc-tick)/1000)
	test(&output)
}

func nine_queens(new_queen_column int8, queen_rows *[N]int8, output *[][N]int8) {
	for new_queen_row := int8(0); new_queen_row < N; new_queen_row++ {
		valid := true
		diagonal_down := new_queen_column + new_queen_row
		diagonal_up := new_queen_column - new_queen_row
		for existing_queen_column := int8(0); existing_queen_column < new_queen_column; existing_queen_column++ {
			existing_queen_row := queen_rows[existing_queen_column]
			if (new_queen_row == existing_queen_row) ||
				(diagonal_down == (existing_queen_column + existing_queen_row)) ||
				(diagonal_up == (existing_queen_column - existing_queen_row)) {
				// solution invalid
				valid = false
				break
			}
		}
		if valid == true {
			// next queen is valid
			queen_rows[new_queen_column] = new_queen_row
			if new_queen_column == N-1 {
				*output = append(*output, *queen_rows) // solution valid
			} else {
				// solution incomplete
				nine_queens(new_queen_column+1, queen_rows, output)
			}
		}
	}
}

func print_square(queen bool, white bool) {
	piece := " "
	if queen {
		piece = "Q"
	}
	color := "1;30;107"
	if white {
		color = "1;97;40"
	}
	fmt.Printf("\x1B[%sm %s \x1B[0m", color, piece)
}

func print_board(board [N]int8) {
	for column := 0; column < N; column++ {
		for row := 0; row < N; row++ {
			print_square(
				bool(int(board[row]) == column),
				bool((column+row)%2 == 0),
			)
		}
		fmt.Print("\x1B[0m\n")
	}
}

func test(output *[][N]int8) {
	if N == 8 {
		fmt.Println(fmt.Sprintln(output) == fmt.Sprintln(&[][8]int8{
			{0, 4, 7, 5, 2, 6, 1, 3}, {0, 5, 7, 2, 6, 3, 1, 4},
			{0, 6, 3, 5, 7, 1, 4, 2}, {0, 6, 4, 7, 1, 3, 5, 2},
			{1, 3, 5, 7, 2, 0, 6, 4}, {1, 4, 6, 0, 2, 7, 5, 3},
			{1, 4, 6, 3, 0, 7, 5, 2}, {1, 5, 0, 6, 3, 7, 2, 4},
			{1, 5, 7, 2, 0, 3, 6, 4}, {1, 6, 2, 5, 7, 4, 0, 3},
			{1, 6, 4, 7, 0, 3, 5, 2}, {1, 7, 5, 0, 2, 4, 6, 3},
			{2, 0, 6, 4, 7, 1, 3, 5}, {2, 4, 1, 7, 0, 6, 3, 5},
			{2, 4, 1, 7, 5, 3, 6, 0}, {2, 4, 6, 0, 3, 1, 7, 5},
			{2, 4, 7, 3, 0, 6, 1, 5}, {2, 5, 1, 4, 7, 0, 6, 3},
			{2, 5, 1, 6, 0, 3, 7, 4}, {2, 5, 1, 6, 4, 0, 7, 3},
			{2, 5, 3, 0, 7, 4, 6, 1}, {2, 5, 3, 1, 7, 4, 6, 0},
			{2, 5, 7, 0, 3, 6, 4, 1}, {2, 5, 7, 0, 4, 6, 1, 3},
			{2, 5, 7, 1, 3, 0, 6, 4}, {2, 6, 1, 7, 4, 0, 3, 5},
			{2, 6, 1, 7, 5, 3, 0, 4}, {2, 7, 3, 6, 0, 5, 1, 4},
			{3, 0, 4, 7, 1, 6, 2, 5}, {3, 0, 4, 7, 5, 2, 6, 1},
			{3, 1, 4, 7, 5, 0, 2, 6}, {3, 1, 6, 2, 5, 7, 0, 4},
			{3, 1, 6, 2, 5, 7, 4, 0}, {3, 1, 6, 4, 0, 7, 5, 2},
			{3, 1, 7, 4, 6, 0, 2, 5}, {3, 1, 7, 5, 0, 2, 4, 6},
			{3, 5, 0, 4, 1, 7, 2, 6}, {3, 5, 7, 1, 6, 0, 2, 4},
			{3, 5, 7, 2, 0, 6, 4, 1}, {3, 6, 0, 7, 4, 1, 5, 2},
			{3, 6, 2, 7, 1, 4, 0, 5}, {3, 6, 4, 1, 5, 0, 2, 7},
			{3, 6, 4, 2, 0, 5, 7, 1}, {3, 7, 0, 2, 5, 1, 6, 4},
			{3, 7, 0, 4, 6, 1, 5, 2}, {3, 7, 4, 2, 0, 6, 1, 5},
			{4, 0, 3, 5, 7, 1, 6, 2}, {4, 0, 7, 3, 1, 6, 2, 5},
			{4, 0, 7, 5, 2, 6, 1, 3}, {4, 1, 3, 5, 7, 2, 0, 6},
			{4, 1, 3, 6, 2, 7, 5, 0}, {4, 1, 5, 0, 6, 3, 7, 2},
			{4, 1, 7, 0, 3, 6, 2, 5}, {4, 2, 0, 5, 7, 1, 3, 6},
			{4, 2, 0, 6, 1, 7, 5, 3}, {4, 2, 7, 3, 6, 0, 5, 1},
			{4, 6, 0, 2, 7, 5, 3, 1}, {4, 6, 0, 3, 1, 7, 5, 2},
			{4, 6, 1, 3, 7, 0, 2, 5}, {4, 6, 1, 5, 2, 0, 3, 7},
			{4, 6, 1, 5, 2, 0, 7, 3}, {4, 6, 3, 0, 2, 7, 5, 1},
			{4, 7, 3, 0, 2, 5, 1, 6}, {4, 7, 3, 0, 6, 1, 5, 2},
			{5, 0, 4, 1, 7, 2, 6, 3}, {5, 1, 6, 0, 2, 4, 7, 3},
			{5, 1, 6, 0, 3, 7, 4, 2}, {5, 2, 0, 6, 4, 7, 1, 3},
			{5, 2, 0, 7, 3, 1, 6, 4}, {5, 2, 0, 7, 4, 1, 3, 6},
			{5, 2, 4, 6, 0, 3, 1, 7}, {5, 2, 4, 7, 0, 3, 1, 6},
			{5, 2, 6, 1, 3, 7, 0, 4}, {5, 2, 6, 1, 7, 4, 0, 3},
			{5, 2, 6, 3, 0, 7, 1, 4}, {5, 3, 0, 4, 7, 1, 6, 2},
			{5, 3, 1, 7, 4, 6, 0, 2}, {5, 3, 6, 0, 2, 4, 1, 7},
			{5, 3, 6, 0, 7, 1, 4, 2}, {5, 7, 1, 3, 0, 6, 4, 2},
			{6, 0, 2, 7, 5, 3, 1, 4}, {6, 1, 3, 0, 7, 4, 2, 5},
			{6, 1, 5, 2, 0, 3, 7, 4}, {6, 2, 0, 5, 7, 4, 1, 3},
			{6, 2, 7, 1, 4, 0, 5, 3}, {6, 3, 1, 4, 7, 0, 2, 5},
			{6, 3, 1, 7, 5, 0, 2, 4}, {6, 4, 2, 0, 5, 7, 1, 3},
			{7, 1, 3, 0, 6, 4, 2, 5}, {7, 1, 4, 2, 0, 6, 3, 5},
			{7, 2, 0, 5, 1, 4, 6, 3}, {7, 3, 0, 2, 5, 1, 6, 4},
		}))
	}
}
