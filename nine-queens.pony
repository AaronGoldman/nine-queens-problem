use "collections"
use "time"

actor Main
  let n: U8 = 9
  let max_print: U64 = 1

  new create(env: Env) =>
    let tic: U64 = Time.nanos()
    let count: U64 = try
      nine_queens(env, 0, Array[U8].init(0, n.usize()), 0)?
    else
      0
    end
    let toc: U64 = Time.nanos()
    env.out.print(
      n.string()+
      " queens "+
      count.string()+
      " solutions in "+
      ((toc-tic)/1000).string()+
      " μs\n"
    )

  fun nine_queens(env: Env, new_queen_column: U8, queen_rows: Array[U8], input: U64): U64? =>
    var output = input
    for new_queen_row in Range[U8](0, n) do
      var valid = true
      let diagonal_down = (new_queen_column + new_queen_row)
      let diagonal_up = (new_queen_column - new_queen_row)
      for existing_queen_column in Range[U8](0, new_queen_column) do
        let existing_queen_row = queen_rows(existing_queen_column.usize())?
        if
          (new_queen_row == existing_queen_row) or
          (diagonal_down == (existing_queen_column + existing_queen_row)) or
          (diagonal_up == (existing_queen_column - existing_queen_row))
        then
          // solution invalid
          valid = false
          break
        end
      end
      if valid then
        // next queen is valid
        queen_rows(new_queen_column.usize())? = new_queen_row
        if new_queen_column == (n-1) then
          // solution valid
          if output < max_print then
            env.out.print("Solution " + output.string())
            print_board(env, queen_rows)
            env.out.print("")
          end
          return output + 1
        else
          // solution incomplete
          output = nine_queens(env, new_queen_column+1, queen_rows, output)?
        end
      end
    end
    output

  fun print_square(env: Env, queen: Bool, white: Bool) =>
    let piece = if queen then "Q" else " " end
    let color = if white then "1;30;107" else "1;97;40" end
    env.out.write("\x1B[" + color + "m " + piece + " ")

  fun print_board(env: Env, board: Array[U8]) =>
    for column in Range[U8](0, n) do
      for row in Range[U8](0, n) do
        let queen = try board(row.usize())? else -1 end
        print_square(env, queen == column, ((column + row) % 2) == 0)
      end
      env.out.print("\x1B[0m") // reset color
    end
