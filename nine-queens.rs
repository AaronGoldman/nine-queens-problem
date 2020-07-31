use std::time::SystemTime;

const         N: i8    = 9;
const MAX_PRINT: usize = 1;

// https://oeis.org/A000170
//  0 queens           1 solutions in    0     ns real  0m  0.198s
//  1 queens           1 solutions in    0     ns real  0m  0.128s
//  2 queens           0 solutions in    0     ns real  0m  0.221s
//  3 queens           0 solutions in    0     ns real  0m  0.124s
//  4 queens           2 solutions in    1     µs real  0m  0.125s
//  5 queens          10 solutions in    5     µs real  0m  0.160s
//  6 queens           4 solutions in   11     µs real  0m  0.117s
//  7 queens          40 solutions in   43     µs real  0m  0.121s
//  8 queens          92 solutions in  196     µs real  0m  0.161s
//  9 queens         352 solutions in  931     µs real  0m  0.109s
// 10 queens         724 solutions in    4.344 ms real  0m  0.118s
// 11 queens       2,680 solutions in   24.029 ms real  0m  0.192s
// 12 queens      14,200 solutions in  140.459 ms real  0m  0.252s
// 13 queens      73,712 solutions in  794.158 ms real  0m  0.909s
// 14 queens     365,596 solutions in    4.782 s  real  0m  4.952s
// 15 queens   2,279,184 solutions in   32.299 s  real  0m 32.447s
// 16 queens  14,772,512 solutions in  229.281 s  real  3m 49.474s
// 17 queens  95,815,104 solutions in 1877.087 s  real 31m 17.398s
// 18 queens 666,090,624 solutions in    3.983 h

fn main() {
    let mut output = Vec::new();
    let mut queen_rows = [0; N as usize];
    let tick = SystemTime::now();
    nine_queens(0, &mut queen_rows, &mut output);
    let toc = tick.elapsed().unwrap();
    for i in 0..(if MAX_PRINT<output.len() {MAX_PRINT} else {output.len()}) {
        println!("Solution {}", i);
        print_board(&output[i]);
    }
    println!("{} queens {} solutions in {:?}", N, output.len(), toc);
    // test(&output);
}

fn nine_queens(
    new_queen_column: i8,
    mut queen_rows: &mut [i8; N as usize],
    mut output: &mut Vec<[i8; N as usize]>,
) {
    // if output.len() > 1 {return}
    for new_queen_row in 0..N {
        let mut valid = true;
        let diagonal_down = new_queen_column + new_queen_row;
        let diagonal_up = new_queen_column - new_queen_row;
        for existing_queen_column in 0..new_queen_column {
            let existing_queen_row = queen_rows[existing_queen_column as usize];
            if new_queen_row     == existing_queen_row
                || diagonal_down == existing_queen_column + existing_queen_row
                || diagonal_up   == existing_queen_column - existing_queen_row
            {
                // solution invalid
                valid = false;
                break;
            }
        }
        if valid == true {
            // next queen is valid
            queen_rows[new_queen_column as usize] = new_queen_row;
            if new_queen_column == N-1 {
                output.push(queen_rows.clone());  // solution valid
            } else {
                // solution incomplete
                nine_queens(new_queen_column + 1, &mut queen_rows, &mut output);
            }
        }
    }
}

fn print_square(queen: bool, white: bool){
    let piece = if queen {"Q"} else {" "};
    let color = if white {"1;30;107"} else {"1;97;40"};
    print!("\x1B[{}m {} \x1B[0m", color, piece)
}

fn print_board(board: &[i8; N as usize]){
    for column in 0..N {
        for row in 0..N as usize{
            print_square(
                board[row] == column,
                (column as usize + row) % 2 == 0,
            );
        }
        print!("\x1B[0m\n");
    }
}

// fn test(output: &Vec<[i8; 8]>) {
//     assert!(output == &vec![
//         [0, 4, 7, 5, 2, 6, 1, 3], [0, 5, 7, 2, 6, 3, 1, 4],
//         [0, 6, 3, 5, 7, 1, 4, 2], [0, 6, 4, 7, 1, 3, 5, 2],
//         [1, 3, 5, 7, 2, 0, 6, 4], [1, 4, 6, 0, 2, 7, 5, 3],
//         [1, 4, 6, 3, 0, 7, 5, 2], [1, 5, 0, 6, 3, 7, 2, 4],
//         [1, 5, 7, 2, 0, 3, 6, 4], [1, 6, 2, 5, 7, 4, 0, 3],
//         [1, 6, 4, 7, 0, 3, 5, 2], [1, 7, 5, 0, 2, 4, 6, 3],
//         [2, 0, 6, 4, 7, 1, 3, 5], [2, 4, 1, 7, 0, 6, 3, 5],
//         [2, 4, 1, 7, 5, 3, 6, 0], [2, 4, 6, 0, 3, 1, 7, 5],
//         [2, 4, 7, 3, 0, 6, 1, 5], [2, 5, 1, 4, 7, 0, 6, 3],
//         [2, 5, 1, 6, 0, 3, 7, 4], [2, 5, 1, 6, 4, 0, 7, 3],
//         [2, 5, 3, 0, 7, 4, 6, 1], [2, 5, 3, 1, 7, 4, 6, 0],
//         [2, 5, 7, 0, 3, 6, 4, 1], [2, 5, 7, 0, 4, 6, 1, 3],
//         [2, 5, 7, 1, 3, 0, 6, 4], [2, 6, 1, 7, 4, 0, 3, 5],
//         [2, 6, 1, 7, 5, 3, 0, 4], [2, 7, 3, 6, 0, 5, 1, 4],
//         [3, 0, 4, 7, 1, 6, 2, 5], [3, 0, 4, 7, 5, 2, 6, 1],
//         [3, 1, 4, 7, 5, 0, 2, 6], [3, 1, 6, 2, 5, 7, 0, 4],
//         [3, 1, 6, 2, 5, 7, 4, 0], [3, 1, 6, 4, 0, 7, 5, 2],
//         [3, 1, 7, 4, 6, 0, 2, 5], [3, 1, 7, 5, 0, 2, 4, 6],
//         [3, 5, 0, 4, 1, 7, 2, 6], [3, 5, 7, 1, 6, 0, 2, 4],
//         [3, 5, 7, 2, 0, 6, 4, 1], [3, 6, 0, 7, 4, 1, 5, 2],
//         [3, 6, 2, 7, 1, 4, 0, 5], [3, 6, 4, 1, 5, 0, 2, 7],
//         [3, 6, 4, 2, 0, 5, 7, 1], [3, 7, 0, 2, 5, 1, 6, 4],
//         [3, 7, 0, 4, 6, 1, 5, 2], [3, 7, 4, 2, 0, 6, 1, 5],
//         [4, 0, 3, 5, 7, 1, 6, 2], [4, 0, 7, 3, 1, 6, 2, 5],
//         [4, 0, 7, 5, 2, 6, 1, 3], [4, 1, 3, 5, 7, 2, 0, 6],
//         [4, 1, 3, 6, 2, 7, 5, 0], [4, 1, 5, 0, 6, 3, 7, 2],
//         [4, 1, 7, 0, 3, 6, 2, 5], [4, 2, 0, 5, 7, 1, 3, 6],
//         [4, 2, 0, 6, 1, 7, 5, 3], [4, 2, 7, 3, 6, 0, 5, 1],
//         [4, 6, 0, 2, 7, 5, 3, 1], [4, 6, 0, 3, 1, 7, 5, 2],
//         [4, 6, 1, 3, 7, 0, 2, 5], [4, 6, 1, 5, 2, 0, 3, 7],
//         [4, 6, 1, 5, 2, 0, 7, 3], [4, 6, 3, 0, 2, 7, 5, 1],
//         [4, 7, 3, 0, 2, 5, 1, 6], [4, 7, 3, 0, 6, 1, 5, 2],
//         [5, 0, 4, 1, 7, 2, 6, 3], [5, 1, 6, 0, 2, 4, 7, 3],
//         [5, 1, 6, 0, 3, 7, 4, 2], [5, 2, 0, 6, 4, 7, 1, 3],
//         [5, 2, 0, 7, 3, 1, 6, 4], [5, 2, 0, 7, 4, 1, 3, 6],
//         [5, 2, 4, 6, 0, 3, 1, 7], [5, 2, 4, 7, 0, 3, 1, 6],
//         [5, 2, 6, 1, 3, 7, 0, 4], [5, 2, 6, 1, 7, 4, 0, 3],
//         [5, 2, 6, 3, 0, 7, 1, 4], [5, 3, 0, 4, 7, 1, 6, 2],
//         [5, 3, 1, 7, 4, 6, 0, 2], [5, 3, 6, 0, 2, 4, 1, 7],
//         [5, 3, 6, 0, 7, 1, 4, 2], [5, 7, 1, 3, 0, 6, 4, 2],
//         [6, 0, 2, 7, 5, 3, 1, 4], [6, 1, 3, 0, 7, 4, 2, 5],
//         [6, 1, 5, 2, 0, 3, 7, 4], [6, 2, 0, 5, 7, 4, 1, 3],
//         [6, 2, 7, 1, 4, 0, 5, 3], [6, 3, 1, 4, 7, 0, 2, 5],
//         [6, 3, 1, 7, 5, 0, 2, 4], [6, 4, 2, 0, 5, 7, 1, 3],
//         [7, 1, 3, 0, 6, 4, 2, 5], [7, 1, 4, 2, 0, 6, 3, 5],
//         [7, 2, 0, 5, 1, 4, 6, 3], [7, 3, 0, 2, 5, 1, 6, 4],
//     ]);
// }
