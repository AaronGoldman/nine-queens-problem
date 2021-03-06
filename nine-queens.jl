#! /usr/bin/env julia
const N = 9
const MAX_PRINT = 1

# https://oeis.org/A000170
# 00 queens          1 solutions in   0.064 µs
# 01 queens          1 solutions in   1.242 µs
# 02 queens          0 solutions in   0.173 µs
# 03 queens          0 solutions in   0.411 µs
# 04 queens          2 solutions in   2.053 µs
# 05 queens         10 solutions in   5.955 µs
# 06 queens          4 solutions in  12.775 µs
# 07 queens         40 solutions in  54.689 µs
# 08 queens         92 solutions in 201.661 µs
# 09 queens        352 solutions in 967.853 µs
# 10 queens        724 solutions in   5.307 ms
# 11 queens      2,680 solutions in  27.519 ms
# 12 queens     14,200 solutions in 147.147 ms
# 13 queens     73,712 solutions in 887.237 ms
# 14 queens    365,596 solutions in   5.658 s
# 15 queens  2,279,184 solutions in  32.423 s
# 16 queens 14,772,512 solutions in   4.018 m
# 17 queens 95,815,104 solutions in  35.419 m

function main()
    output = []
    queen_rows = ones(Int8, N)
    elapsed = @elapsed nine_queens(1, queen_rows, output)
    for i in 1:min(MAX_PRINT, length(output))
        println("Solution ", i)
        print_board(output[i])
    end
    println(N, " queens ", length(output), " solutions in ", elapsed * 1000000, "µs")
    if N == 8 
        test(output)
    end
end

function nine_queens(new_queen_column, queen_rows, output)
    for new_queen_row in 1:N
        valid = true
        diagonal_down = new_queen_column + new_queen_row;
        diagonal_up = new_queen_column - new_queen_row;
        for existing_queen_column in 1:new_queen_column-1
            existing_queen_row = queen_rows[existing_queen_column];
            if ( 
                new_queen_row == existing_queen_row 
                || diagonal_down == existing_queen_column + existing_queen_row 
                || diagonal_up == existing_queen_column - existing_queen_row
            )
                valid = false
                break # solution invalid
            end
        end

        if valid == true  # next queen is valid
            queen_rows[new_queen_column] = new_queen_row;
            if new_queen_column == N
                push!(output, copy(queen_rows)) # solution valid
            else
                nine_queens(new_queen_column + 1, queen_rows, output)
            end
        end
    end
end

function print_square(queen, white)
    piece = queen ? "Q"        : " "
    color = white ? "1;30;107" : "1;97;40"
    print("\x1B[", color, "m ", piece, " \x1B[0m")
end

function print_board(board)
    for column in 1:N
        for row in 1:N
            print_square(
                board[row] == column,
                (column + row) % 2 == 0,
            );
        end
        println("")
    end
end

function test(output)
    expected = [
        [1, 5, 8, 6, 3, 7, 2, 4], [1, 6, 8, 3, 7, 4, 2, 5],
        [1, 7, 4, 6, 8, 2, 5, 3], [1, 7, 5, 8, 2, 4, 6, 3],
        [2, 4, 6, 8, 3, 1, 7, 5], [2, 5, 7, 1, 3, 8, 6, 4],
        [2, 5, 7, 4, 1, 8, 6, 3], [2, 6, 1, 7, 4, 8, 3, 5],
        [2, 6, 8, 3, 1, 4, 7, 5], [2, 7, 3, 6, 8, 5, 1, 4],
        [2, 7, 5, 8, 1, 4, 6, 3], [2, 8, 6, 1, 3, 5, 7, 4],
        [3, 1, 7, 5, 8, 2, 4, 6], [3, 5, 2, 8, 1, 7, 4, 6],
        [3, 5, 2, 8, 6, 4, 7, 1], [3, 5, 7, 1, 4, 2, 8, 6],
        [3, 5, 8, 4, 1, 7, 2, 6], [3, 6, 2, 5, 8, 1, 7, 4],
        [3, 6, 2, 7, 1, 4, 8, 5], [3, 6, 2, 7, 5, 1, 8, 4],
        [3, 6, 4, 1, 8, 5, 7, 2], [3, 6, 4, 2, 8, 5, 7, 1],
        [3, 6, 8, 1, 4, 7, 5, 2], [3, 6, 8, 1, 5, 7, 2, 4],
        [3, 6, 8, 2, 4, 1, 7, 5], [3, 7, 2, 8, 5, 1, 4, 6],
        [3, 7, 2, 8, 6, 4, 1, 5], [3, 8, 4, 7, 1, 6, 2, 5],
        [4, 1, 5, 8, 2, 7, 3, 6], [4, 1, 5, 8, 6, 3, 7, 2],
        [4, 2, 5, 8, 6, 1, 3, 7], [4, 2, 7, 3, 6, 8, 1, 5],
        [4, 2, 7, 3, 6, 8, 5, 1], [4, 2, 7, 5, 1, 8, 6, 3],
        [4, 2, 8, 5, 7, 1, 3, 6], [4, 2, 8, 6, 1, 3, 5, 7],
        [4, 6, 1, 5, 2, 8, 3, 7], [4, 6, 8, 2, 7, 1, 3, 5],
        [4, 6, 8, 3, 1, 7, 5, 2], [4, 7, 1, 8, 5, 2, 6, 3],
        [4, 7, 3, 8, 2, 5, 1, 6], [4, 7, 5, 2, 6, 1, 3, 8],
        [4, 7, 5, 3, 1, 6, 8, 2], [4, 8, 1, 3, 6, 2, 7, 5],
        [4, 8, 1, 5, 7, 2, 6, 3], [4, 8, 5, 3, 1, 7, 2, 6],
        [5, 1, 4, 6, 8, 2, 7, 3], [5, 1, 8, 4, 2, 7, 3, 6],
        [5, 1, 8, 6, 3, 7, 2, 4], [5, 2, 4, 6, 8, 3, 1, 7],
        [5, 2, 4, 7, 3, 8, 6, 1], [5, 2, 6, 1, 7, 4, 8, 3],
        [5, 2, 8, 1, 4, 7, 3, 6], [5, 3, 1, 6, 8, 2, 4, 7],
        [5, 3, 1, 7, 2, 8, 6, 4], [5, 3, 8, 4, 7, 1, 6, 2],
        [5, 7, 1, 3, 8, 6, 4, 2], [5, 7, 1, 4, 2, 8, 6, 3],
        [5, 7, 2, 4, 8, 1, 3, 6], [5, 7, 2, 6, 3, 1, 4, 8],
        [5, 7, 2, 6, 3, 1, 8, 4], [5, 7, 4, 1, 3, 8, 6, 2],
        [5, 8, 4, 1, 3, 6, 2, 7], [5, 8, 4, 1, 7, 2, 6, 3],
        [6, 1, 5, 2, 8, 3, 7, 4], [6, 2, 7, 1, 3, 5, 8, 4],
        [6, 2, 7, 1, 4, 8, 5, 3], [6, 3, 1, 7, 5, 8, 2, 4],
        [6, 3, 1, 8, 4, 2, 7, 5], [6, 3, 1, 8, 5, 2, 4, 7],
        [6, 3, 5, 7, 1, 4, 2, 8], [6, 3, 5, 8, 1, 4, 2, 7],
        [6, 3, 7, 2, 4, 8, 1, 5], [6, 3, 7, 2, 8, 5, 1, 4],
        [6, 3, 7, 4, 1, 8, 2, 5], [6, 4, 1, 5, 8, 2, 7, 3],
        [6, 4, 2, 8, 5, 7, 1, 3], [6, 4, 7, 1, 3, 5, 2, 8],
        [6, 4, 7, 1, 8, 2, 5, 3], [6, 8, 2, 4, 1, 7, 5, 3],
        [7, 1, 3, 8, 6, 4, 2, 5], [7, 2, 4, 1, 8, 5, 3, 6],
        [7, 2, 6, 3, 1, 4, 8, 5], [7, 3, 1, 6, 8, 5, 2, 4],
        [7, 3, 8, 2, 5, 1, 6, 4], [7, 4, 2, 5, 8, 1, 3, 6],
        [7, 4, 2, 8, 6, 1, 3, 5], [7, 5, 3, 1, 6, 8, 2, 4],
        [8, 2, 4, 1, 7, 5, 3, 6], [8, 2, 5, 3, 1, 7, 4, 6],
        [8, 3, 1, 6, 2, 5, 7, 4], [8, 4, 1, 3, 6, 2, 7, 5],
    ]

    for i in 1:length(output)
        if output[i] != expected[i]
            println("expected ", i)
            print_board(expected[i])
            println("got ", i)
            println(output[i])
            print_board(output[i])
            println()
            break
        end
    end
end

main()
