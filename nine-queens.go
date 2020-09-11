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
//  0 queens             0 solutions in              0 μs real        0.141s
//  1 queens             1 solutions in             41 μs real        0.124s
//  2 queens             0 solutions in              1 μs real        0.161s
//  3 queens             0 solutions in              1 μs real        0.130s
//  4 queens             2 solutions in             65 μs real        0.215s
//  5 queens            10 solutions in             99 μs real        0.132s
//  6 queens             4 solutions in            136 μs real        0.216s
//  7 queens            40 solutions in            159 μs real        0.212s
//  8 queens            92 solutions in            339 μs real        0.258s
//  9 queens           352 solutions in          1,219 μs real        0.206s
// 10 queens           724 solutions in          5,079 μs real        0.200s
// 11 queens         2,680 solutions in         23,362 μs real        0.213s
// 12 queens        14,200 solutions in        119,652 μs real        0.253s
// 13 queens        73,712 solutions in        740,570 μs real        0.953s
// 14 queens       365,596 solutions in      4,435,962 μs real        4.565s
// 15 queens     2,279,184 solutions in     28,342,354 μs real     0m28.842s
// 16 queens    14,772,512 solutions in    198,337,612 μs real     3m18.923s
// 17 queens    95,815,104 solutions in  1,547,927,042 μs real    25m48.315s
// 18 queens   666,090,624 solutions in 11,823,132,011 μs real  3h17m04.074s
// 19 queens 4,968,057,848 solutions in 85,752,404,290 μs real 23h49m12.706s

func main() {
	tick := time.Now().UnixNano()
	output := nine_queens(0, [N]int8{}, int64(0))
	toc := time.Now().UnixNano()

	fmt.Printf("%v queens %v solutions in %d μs\n", N, output, (toc-tick)/1000)
	test(output)
}

func nine_queens(new_queen_column int8, queen_rows [N]int8, output int64) int64 {
	for new_queen_row := int8(0); new_queen_row < N; new_queen_row++ {
		valid := true
		diagonal_down := new_queen_column + new_queen_row
		diagonal_up := new_queen_column - new_queen_row
		for existing_queen_column := int8(0); existing_queen_column < new_queen_column; existing_queen_column++ {
			existing_queen_row := queen_rows[existing_queen_column]
			if new_queen_row == existing_queen_row ||
				diagonal_down == existing_queen_column+existing_queen_row ||
				diagonal_up == existing_queen_column-existing_queen_row {
				// solution invalid
				valid = false
				break
			}
		}
		if valid {
			// next queen is valid
			queen_rows[new_queen_column] = new_queen_row
			if new_queen_column == N-1 {
				// solution valid
				if output < MAX_PRINT {
					fmt.Println("Solution ", output)
					print_board(queen_rows)
					fmt.Println()
				}
				return output + 1
			} else {
				// solution incomplete
				output = nine_queens(new_queen_column+1, queen_rows, output)
			}
		}
	}
	return output
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
	for column := int8(0); column < N; column++ {
		for row := int8(0); row < N; row++ {
			print_square(
				board[row] == column,
				(column+row)%2 == 0,
			)
		}
		fmt.Print("\x1B[0m\n")
	}
}

func test(output int64) {
	expected, ok := map[int8]int64{
		0:  1,
		1:  1,
		2:  0,
		3:  0,
		4:  2,
		5:  10,
		6:  4,
		7:  40,
		8:  92,
		9:  352,
		10: 724,
		11: 2680,
		12: 14200,
		13: 73712,
		14: 365596,
		15: 2279184,
		16: 14772512,
		17: 95815104,
		18: 666090624,
		19: 4968057848,
		20: 39029188884,
		21: 314666222712,
		22: 2691008701644,
		23: 24233937684440,
		24: 227514171973736,
		25: 2207893435808352,
		26: 22317699616364044,
		27: 234907967154122528,
	}[N]
	if ok {
		if output == expected {
			fmt.Println("Pass")
		} else {
			fmt.Println("Fail", "expected", expected, "output", output)
		}
	}
}
