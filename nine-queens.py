#! /usr/bin/env python3
from time import time

N = 9

# 00 queens         1 solutions in 4 µs
# 01 queens         1 solutions in 7 µs
# 02 queens         0 solutions in 8 µs
# 03 queens         0 solutions in 16 µs
# 04 queens         0 solutions in 25 µs
# 05 queens        10 solutions in 184 µs
# 06 queens         4 solutions in 697 µs
# 07 queens        40 solutions in 2.988 ms
# 08 queens        92 solutions in 13.145 ms
# 09 queens       352 solutions in 62.520 ms
# 10 queens       724 solutions in 302.486 ms
# 11 queens     2,680 solutions in 1.686 s
# 12 queens    14,200 solutions in 10.415 s
# 13 queens    73,712 solutions in 64.990 s
# 14 queens   365,596 solutions in 7.611 m
# 15 queens 2,279,184 solutions in 1.420 h


def main():
    output = []
    queen_rows = [0] * N

    tic = time()
    nine_queens(0, queen_rows, output)
    toc = time()
    elapsed = toc - tic 

    for i in range(1):
        print("Solution ", i)
        print_board(output[i])

    print(f"{N} queens {len(output)} solutions in {int(elapsed * 1000000)} µs")
    if N == 8: test(output)

def nine_queens(new_queen_column, queen_rows, output):
    for new_queen_row in range(N):
        valid = True
        diagonal_down = new_queen_column + new_queen_row;
        diagonal_up = new_queen_column - new_queen_row;
        for existing_queen_column in range(new_queen_column):
            existing_queen_row = queen_rows[existing_queen_column];
            if new_queen_row == existing_queen_row or \
               diagonal_down == existing_queen_column + existing_queen_row or \
               diagonal_up == existing_queen_column - existing_queen_row:
                valid = False
                break # solution invalid

        if valid == True:
            # next queen is valid
            queen_rows[new_queen_column] = new_queen_row;
            if new_queen_column == N - 1:
                output.append(queen_rows.copy())  # solution valid
            else:
                nine_queens(new_queen_column + 1, queen_rows, output)


def print_square(s, color):
    if color:
        print(f"\x1B[1;30;107m {s} \x1B[0m", end="")
    else:
        print(f"\x1B[1;97;40m {s} \x1B[0m", end="")


def print_board(board):
    for column in range(N):
        for row in range(N):
            if board[row] == column:
                print_square("Q", (column + row) % 2 == 0);
            else:
                print_square(" ", (column + row) % 2 == 0);
        print("")


def test(output):
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

    for i in range(len(output)):
        if output[i] != expected[i]:
            print("expected ", i)
            print_board(expected[i])
            print("got ", i)
            print(output[i])
            print_board(output[i])
            print()
            break

if __name__ == "__main__":
    main()

