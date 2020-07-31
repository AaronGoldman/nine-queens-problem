// time javac NineQueens.java
// time java NineQueens
//  0 queens           1 solutions in              1 µs real         0.145s
//  1 queens           1 solutions in            198 µs real         0.137s
//  2 queens           0 solutions in              2 µs real         0.150s
//  3 queens           0 solutions in              4 µs real         0.149s
//  4 queens           2 solutions in            384 µs real         0.145s
//  5 queens          10 solutions in            532 µs real         0.146s
//  6 queens           4 solutions in            705 µs real         0.144s
//  7 queens          40 solutions in          1,124 µs real         0.151s
//  8 queens          92 solutions in          1,989 µs real         0.148s
//  9 queens         352 solutions in          3,692 µs real         0.145s
// 10 queens         724 solutions in         12,960 µs real         0.161s
// 11 queens       2,680 solutions in         37,086 µs real         0.183s
// 12 queens      14,200 solutions in        147,173 µs real         0.294s
// 13 queens      73,712 solutions in        722,837 µs real         0.865s
// 14 queens     365,596 solutions in      4,649,648 µs real         4.791s
// 15 queens   2,279,184 solutions in     33,155,702 µs real        33.300s
// 16 queens  14,772,512 solutions in    243,151,529 µs real     4m  3.282s
// 17 queens  95,815,104 solutions in  1,781,271,867 µs real    29m 41.447s
// 18 queens 666,090,624 solutions in 12,728,976,542 µs real 3h 32m  9.209s
// 19 4,968,057,848

public class NineQueens {
  private static int   N          = 9;
  private static int   MAX_PRINT  = 1;
  private static int[] queen_rows = new int[N];
  private static long  output     = 0;

  public static void nine_queens(int new_queen_column){
    for (int new_queen_row=0; new_queen_row<N; new_queen_row++){
      int diagonal_down = new_queen_column + new_queen_row;
      int diagonal_up   = new_queen_column - new_queen_row;
      boolean valid     = true;
      for (
        int existing_queen_column = 0;
        existing_queen_column < new_queen_column;
        existing_queen_column++
      ){
        int existing_queen_row = queen_rows[existing_queen_column];
        if ( (new_queen_row == existing_queen_row)
          || (diagonal_down == existing_queen_column + existing_queen_row)
          || (diagonal_up   == existing_queen_column - existing_queen_row)
        ){
          valid = false;
          break; // solution invalid
        }
      }
      if (valid) {
        // next queen is valid
        queen_rows[new_queen_column] = new_queen_row;
        if (new_queen_column < N-1){
          nine_queens(new_queen_column + 1);
        } else {
          if (output < MAX_PRINT){
            System.out.print("Solution " + output + "\n");
            print_board(queen_rows);
          }
          output += 1;
        }
      }
    }
  }

  public static void print_square(boolean queen, boolean white){
    String piece = queen ? "Q" : " ";
    String color = white ? "1;30;107" : "1;97;40";
    System.out.print("\u001B[" + color + "m " + piece + " \u001B[0m");
  }

  public static void print_board(int[] board){
    for (int column = 0; column < N; column++){
        for (int row = 0; row < N; row++){
            print_square((board[row] == column), ((column + row) % 2 == 0));
        }
        System.out.println("\u001B[0m");  // reset color at end of row
    }
  }
  public static void main(String[] args){
    for (int i = 0; i < N; i++) {
      queen_rows[i] = 0;
    }
    long tic = System.nanoTime();
    nine_queens(0);
    long toc = System.nanoTime();
    long elapsed = (toc - tic) / 1000;
    System.out.println(
      N + " queens " + output +" solutions in " + elapsed +" µs\n"
    );
  }
}