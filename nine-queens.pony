use "collections"

actor Main
  let n: USize = 8

  new create(env: Env) =>
    print_board(env, [0; 1; 2; 3; 4; 5; 6; 7])

  fun print_square(env: Env, queen: Bool, white: Bool) =>
    let piece = if queen then "Q" else " " end
    let color = if white then "1;30;107" else "1;97;40" end
    env.out.write("\x1B[" + color + " m " + piece + " ")

  fun print_board(env: Env, board: Array[USize]) =>
    for column in Range(0, n) do
      for row in Range(0, n) do
        let queen = try board(row)? else 1 end
        print_square(
            env,
            Bool(queen == column),
            Bool(((column + row) % 2) == 0)
        )
      end
      env.out.print("\x1B[0m") // reset color
    end
