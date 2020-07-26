#! /usr/bin/env node
N = 9
MAX_PRINT = 1

// https://oeis.org/A000170
// 00 queens       1 solutions in 142.000 µs
// 01 queens       1 solutions in 197.000 µs
// 02 queens       0 solutions in 166.000 µs
// 03 queens       0 solutions in 141.000 µs
// 04 queens       2 solutions in 205.000 µs
// 05 queens      10 solutions in 297.000 µs
// 06 queens       4 solutions in 267.000 µs
// 07 queens      40 solutions in   7.078 ms
// 08 queens      92 solutions in   3.985 ms
// 09 queens     352 solutions in   6.066 ms
// 10 queens     724 solutions in  11.647 ms
// 11 queens   2,680 solutions in  56.586 ms
// 12 queens  14,200 solutions in 226.685 ms
// 13 queens  73,712 solutions in   1.309 s
// 14 queens 365,596 solutions in   8.386 s
// 15 queens 2279184 solutions in  60.722 s


function main(){
    let output = [];
    let queen_rows = Array(N).fill(0)

    let tic = process.hrtime.bigint()
    nine_queens(0, queen_rows, output)
    let toc = process.hrtime.bigint()
    elapsed =  (toc - tic) / 1000n

    for (i=0; i<Math.min(MAX_PRINT, output.length); i++){
        console.log("Solution ", i);
        print_board(output[i]);
    }
    console.log(
        N.toString() + 
        " queens " +
        output.length +
        " solutions in " +
        elapsed +
        " µs"
    )
    if ( N == 8 ){ test(output) }
}

function nine_queens(new_queen_column, queen_rows, output){
    for (let new_queen_row = 0; new_queen_row < N; new_queen_row++){
        let valid = true
        let diagonal_down = new_queen_column + new_queen_row;
        let diagonal_up = new_queen_column - new_queen_row;
        for (let existing_queen_column = 0; existing_queen_column < new_queen_column; existing_queen_column++){
            existing_queen_row = queen_rows[existing_queen_column];
            if (
                new_queen_row == existing_queen_row
                || diagonal_down == existing_queen_column + existing_queen_row
                || diagonal_up == existing_queen_column - existing_queen_row
            ){
                valid = false
                break // solution invalid
            }
        }

        if (valid == true) {
            // next queen is valid
            queen_rows[new_queen_column] = new_queen_row;
            if (new_queen_column == N - 1) {
                output.push(queen_rows.slice())  // solution valid
            } else {
                nine_queens(new_queen_column + 1, queen_rows, output)
            }
        }
    }
}


function print_square(s, color){
    if (color) {
        process.stdout.write("\x1B[1;30;107m " + s + " \x1B[0m")
    } else {
        process.stdout.write("\x1B[1;97;40m " + s + " \x1B[0m")
    }
}

function print_board(board){
    for (let column = 0; column < N; column++){
        for (let row = 0; row < N; row++){
            if (board[row] == column){
                print_square("Q", (column + row) % 2 == 0)
            } else {
                print_square(" ", (column + row) % 2 == 0)
            }
        }
        console.log("")
    }
}

function test(output){
    expected = [
        [0, 4, 7, 5, 2, 6, 1, 3], [0, 5, 7, 2, 6, 3, 1, 4],
        [0, 6, 3, 5, 7, 1, 4, 2], [0, 6, 4, 7, 1, 3, 5, 2],
        [1, 3, 5, 7, 2, 0, 6, 4], [1, 4, 6, 0, 2, 7, 5, 3],
        [1, 4, 6, 3, 0, 7, 5, 2], [1, 5, 0, 6, 3, 7, 2, 4],
        [1, 5, 7, 2, 0, 3, 6, 4], [1, 6, 2, 5, 7, 4, 0, 3],
        [1, 6, 4, 7, 0, 3, 5, 2], [1, 7, 5, 0, 2, 4, 6, 3],
        [2, 0, 6, 4, 7, 1, 3, 5], [2, 4, 1, 7, 0, 6, 3, 5],
        [2, 4, 1, 7, 5, 3, 6, 0], [2, 4, 6, 0, 3, 1, 7, 5],
        [2, 4, 7, 3, 0, 6, 1, 5], [2, 5, 1, 4, 7, 0, 6, 3],
        [2, 5, 1, 6, 0, 3, 7, 4], [2, 5, 1, 6, 4, 0, 7, 3],
        [2, 5, 3, 0, 7, 4, 6, 1], [2, 5, 3, 1, 7, 4, 6, 0],
        [2, 5, 7, 0, 3, 6, 4, 1], [2, 5, 7, 0, 4, 6, 1, 3],
        [2, 5, 7, 1, 3, 0, 6, 4], [2, 6, 1, 7, 4, 0, 3, 5],
        [2, 6, 1, 7, 5, 3, 0, 4], [2, 7, 3, 6, 0, 5, 1, 4],
        [3, 0, 4, 7, 1, 6, 2, 5], [3, 0, 4, 7, 5, 2, 6, 1],
        [3, 1, 4, 7, 5, 0, 2, 6], [3, 1, 6, 2, 5, 7, 0, 4],
        [3, 1, 6, 2, 5, 7, 4, 0], [3, 1, 6, 4, 0, 7, 5, 2],
        [3, 1, 7, 4, 6, 0, 2, 5], [3, 1, 7, 5, 0, 2, 4, 6],
        [3, 5, 0, 4, 1, 7, 2, 6], [3, 5, 7, 1, 6, 0, 2, 4],
        [3, 5, 7, 2, 0, 6, 4, 1], [3, 6, 0, 7, 4, 1, 5, 2],
        [3, 6, 2, 7, 1, 4, 0, 5], [3, 6, 4, 1, 5, 0, 2, 7],
        [3, 6, 4, 2, 0, 5, 7, 1], [3, 7, 0, 2, 5, 1, 6, 4],
        [3, 7, 0, 4, 6, 1, 5, 2], [3, 7, 4, 2, 0, 6, 1, 5],
        [4, 0, 3, 5, 7, 1, 6, 2], [4, 0, 7, 3, 1, 6, 2, 5],
        [4, 0, 7, 5, 2, 6, 1, 3], [4, 1, 3, 5, 7, 2, 0, 6],
        [4, 1, 3, 6, 2, 7, 5, 0], [4, 1, 5, 0, 6, 3, 7, 2],
        [4, 1, 7, 0, 3, 6, 2, 5], [4, 2, 0, 5, 7, 1, 3, 6],
        [4, 2, 0, 6, 1, 7, 5, 3], [4, 2, 7, 3, 6, 0, 5, 1],
        [4, 6, 0, 2, 7, 5, 3, 1], [4, 6, 0, 3, 1, 7, 5, 2],
        [4, 6, 1, 3, 7, 0, 2, 5], [4, 6, 1, 5, 2, 0, 3, 7],
        [4, 6, 1, 5, 2, 0, 7, 3], [4, 6, 3, 0, 2, 7, 5, 1],
        [4, 7, 3, 0, 2, 5, 1, 6], [4, 7, 3, 0, 6, 1, 5, 2],
        [5, 0, 4, 1, 7, 2, 6, 3], [5, 1, 6, 0, 2, 4, 7, 3],
        [5, 1, 6, 0, 3, 7, 4, 2], [5, 2, 0, 6, 4, 7, 1, 3],
        [5, 2, 0, 7, 3, 1, 6, 4], [5, 2, 0, 7, 4, 1, 3, 6],
        [5, 2, 4, 6, 0, 3, 1, 7], [5, 2, 4, 7, 0, 3, 1, 6],
        [5, 2, 6, 1, 3, 7, 0, 4], [5, 2, 6, 1, 7, 4, 0, 3],
        [5, 2, 6, 3, 0, 7, 1, 4], [5, 3, 0, 4, 7, 1, 6, 2],
        [5, 3, 1, 7, 4, 6, 0, 2], [5, 3, 6, 0, 2, 4, 1, 7],
        [5, 3, 6, 0, 7, 1, 4, 2], [5, 7, 1, 3, 0, 6, 4, 2],
        [6, 0, 2, 7, 5, 3, 1, 4], [6, 1, 3, 0, 7, 4, 2, 5],
        [6, 1, 5, 2, 0, 3, 7, 4], [6, 2, 0, 5, 7, 4, 1, 3],
        [6, 2, 7, 1, 4, 0, 5, 3], [6, 3, 1, 4, 7, 0, 2, 5],
        [6, 3, 1, 7, 5, 0, 2, 4], [6, 4, 2, 0, 5, 7, 1, 3],
        [7, 1, 3, 0, 6, 4, 2, 5], [7, 1, 4, 2, 0, 6, 3, 5],
        [7, 2, 0, 5, 1, 4, 6, 3], [7, 3, 0, 2, 5, 1, 6, 4],
    ]

    for (let i = 0; i < output.length; i++){
        if (""+output[i] != ""+expected[i]){
            console.log("expected ", i)
            console.log(output[i])
            print_board(expected[i])
            console.log("got ", i)
            console.log(output[i])
            print_board(output[i])
            console.log()
            break
        }
    }
}

main()
