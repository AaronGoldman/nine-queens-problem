#! /usr/bin/env awk -f
function main(){
    N = 9;
    MAX_PRINT = 1;

    # for (i=0;i<N;i++){ queen_rows[i] = 0 }
    # existing_queen_column = 0;
    # existing_queen_row = 0;
    # valid = 0;
    # diagonal_down = 0;
    # diagonal_up = 0;

    # tic = time()
    nine_queens(0)
    # elapsed = time() - tic
    printf("%s queens %s solutions in %s µs\n", N, output, "???")
    exit;
}

#  8 queens      92 solutions in ??? µs real  0m  0.043s
#  9 queens     352 solutions in ??? µs real  0m  0.174s
# 10 queens     724 solutions in ??? µs real  0m  0.820s
# 11 queens   2,680 solutions in ??? µs real  0m  4.538s
# 12 queens  14,200 solutions in ??? µs real  0m 27.613s
# 13 queens  73,712 solutions in ??? µs real  2m 58.035s
# 14 queens 365,596 solutions in ??? µs real 20m 26.593s
# EXPECTED 4 h 20 m

function nine_queens(new_queen_column,  new_queen_row){
    for (new_queen_row=0; new_queen_row<N; new_queen_row++){
        valid = 1;
        diagonal_down = new_queen_column + new_queen_row;
        diagonal_up = new_queen_column - new_queen_row;
        for (existing_queen_column=0;
             existing_queen_column<new_queen_column;
             existing_queen_column++){
            existing_queen_row = queen_rows[existing_queen_column];
            if ((new_queen_row == existing_queen_row) ||
              (diagonal_down == existing_queen_column + existing_queen_row) ||
              (diagonal_up   == existing_queen_column - existing_queen_row)){
                valid = 0;
                break # solution invalid
            }
        }
        if (valid) {
            # next queen is valid
            queen_rows[new_queen_column] = new_queen_row;
            if (new_queen_column < N-1){
                nine_queens(new_queen_column + 1)
            } else {
                if (output < MAX_PRINT){
                    printf("Solution %d\n", output);
                    print_board(queen_rows);
                }
                output += 1
            }
        }
    }
}

function print_square(queen, white){
    piece = queen ? "Q" : " ";
    color = white ? "1;30;107" : "1;97;40";
    printf("\x1B[%sm %s ", color, piece);
}

function print_board(board){
    for (column = 0; column < N; column++){
        for (row = 0; row < N; row++){
            print_square((board[row] == column), ((column + row) % 2 == 0));
        }
        print "\x1B[0m";  # reset color at end of row
    }
}

BEGIN {main()}
