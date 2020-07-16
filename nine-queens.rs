use std::time::SystemTime;

const N: i8 = 8;
// N = 00 |           1 solutions in 1µs
// N = 01 |           1 solutions in 1µs
// N = 02 |           0 solutions in 1µs
// N = 03 |           0 solutions in 1µs
// N = 04 |           2 solutions in 2µs
// N = 05 |          10 solutions in 4µs
// N = 06 |           4 solutions in 15µs
// N = 07 |          40 solutions in 42µs
// N = 08 |          92 solutions in 255µs
// N = 09 |         352 solutions in 1.236ms
// N = 10 |         724 solutions in 5.361ms
// N = 11 |       2,680 solutions in 24.662ms
// N = 12 |      14,200 solutions in 130.601ms
// N = 13 |      73,712 solutions in 678.761ms
// N = 14 |     365,596 solutions in 4.373536s
// N = 15 |   2,279,184 solutions in 28.850197s
// N = 16 |  14,772,512 solutions in 3.454m
// N = 17 |  95,815,104 solutions in 30.111m
// N = 18 | 666,090,624 solutions in 3.983 h

fn main() {
    let mut output = Vec::new();
    let mut queen_rows = [0; N as usize];
    let tick = SystemTime::now();
    nine_queens(0, &mut queen_rows, &mut output);
    let toc = tick.elapsed().unwrap();
    for i in 0..1 {
        println!("Solution {}", i);
        print_board(&output[i]);
    }
    println!("{} queens {:?} solutions in {:?}", N, output.len(), toc);
    test(&output);
}

fn nine_queens(
    new_queen_column: i8,
    mut queen_rows: &mut [i8; N as usize],
    mut output: &mut Vec<[i8; N as usize]>,
) {
    for new_queen_row in 0..N {
        let mut valid = true;
        let diagonal_down = new_queen_column + new_queen_row;
        let diagonal_up = new_queen_column - new_queen_row;
        for existing_queen_column in 0..new_queen_column {
            let existing_queen_row = queen_rows[existing_queen_column as usize];
            if new_queen_row == existing_queen_row
                || diagonal_down == existing_queen_column + existing_queen_row
                || diagonal_up == existing_queen_column - existing_queen_row
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

fn print_square(s: &str, color: bool){
    if color {
        print!("\x1B[1;30;107m {} \x1B[0m", s)
    } else {
        print!("\x1B[1;97;40m {} \x1B[0m", s)
    }
}

fn print_board(board: &[i8; N as usize]){
    for column in 0..N {
        for row in 0..N as usize{
            if board[row] == column {
                print_square("Q", (column as usize + row) % 2 == 0);
            } else {
                print_square(" ", (column as usize + row) % 2 == 0);
            }
        }
        print!("\n");
    }
}

fn test(output: &Vec<[i8; 8]>) {
    assert!(output == &vec![
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
    ]);
}
