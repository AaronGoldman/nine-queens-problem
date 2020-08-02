# THE PROBLEM OF THE EIGHT QUEENS

## Solving one problem in many langweges

It is requested to make a program generating all configurations of eight
queens on a chessboard of 8*8 squares such that no queen can take any of
the others. This means that in the configurations sought, no two queens
may be on the same row, on the same column or on the same diagonal. 

https://t.co/LKWZacSTha

| number of queen | number of solutions     | 20 µs * nSolutions |
|-----------------|------------------------:|-------------------:|
|              00 |                       1 |           20.00 µs |
|              01 |                       1 |           20.00 µs |
|              02 |                       0 |            0.00 µs |
|              03 |                       0 |            0.00 µs |
|              04 |                       2 |           40.00 µs |
|              05 |                      10 |          400.00 µs |
|              06 |                       4 |           80.00 µs |
|              07 |                      40 |          800.00 µs |
|              08 |                      92 |            1.84 ms |
|              09 |                     352 |            7.04 ms |
|              10 |                     724 |           14.48 ms |
|              11 |                   2,680 |           53.60 ms |
|              12 |                  14,200 |          284.00 ms |
|              13 |                  73,712 |            1.47 s  |
|              14 |                 365,596 |            7.31 s  |
|              15 |               2,279,184 |           45.58 s  |
|              16 |              14,772,512 |            4.92 m  |
|              17 |              95,815,104 |           31.94 m  |
|              18 |             666,090,624 |            3.70 h  |
|              19 |           4,968,057,848 |            1.15 d  |
|              20 |          39,029,188,884 |            9.03 d  |
|              21 |         314,666,222,712 |           72.83 d  |
|              22 |       2,691,008,701,644 |            1.70 y  |
|              23 |      24,233,937,684,440 |           15.36 y  |
|              24 |     227,514,171,973,736 |          144.19 y  |
|              25 |   2,207,893,435,808,352 |        1,399.28 y  |
|              26 |  22,317,699,616,364,044 |       14,144.10 y  |
|              27 | 234,907,967,154,122,528 |      148,875.69 y  |

## Implimentations
* Python 3
  ```sh
  time python3 nine-queens.py
  time pypy3 nine-queens.py
  ```
* Node
  ```sh
  time node nine-queens.js
  ```
* Julea
  ```sh
  time julia nine-queens.jl
  ```
* Rust
  ```sh
  time rustc -O nine-queens.rs
  time ./nine-queens
  ```
* Awk
  ```sh
  time awk -f nine-queens.awk
  ```
* CLojure
  ```sh
  time java -jar ../clojure/clojure.jar nine-queens.clj
  ```
* Java
  ```sh
  time javac NineQueens.java
  time java NineQueens
  ```