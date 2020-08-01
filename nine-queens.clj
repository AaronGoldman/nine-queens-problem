;; cat nine-queens.clj | java -jar ../clojure/clojure.jar -

;  0 queens         1 solutions in           625 µs real  0m  1.651s
;  1 queens         1 solutions in         1,519 µs real  0m  1.346s
;  2 queens         0 solutions in           336 µs real  0m  1.689s
;  3 queens         0 solutions in           398 µs real  0m  1.444s
;  4 queens         2 solutions in         2,385 µs real  0m  2.169s
;  5 queens        10 solutions in         2,980 µs real  0m  1.398s
;  6 queens         4 solutions in         5,942 µs real  0m  1.338s
;  7 queens        40 solutions in        13,616 µs real  0m  1.454s
;  8 queens        92 solutions in        30,732 µs real  0m  1.458s
;  9 queens       352 solutions in        54,961 µs real  0m  1.738s
; 10 queens       724 solutions in       119,561 µs real  0m  1.643s
; 11 queens     2,680 solutions in       614,423 µs real  0m  2.008s
; 12 queens    14,200 solutions in     3,723,711 µs real  0m  5.076s
; 13 queens    73,712 solutions in    11,775,197 µs real  0m 13.236s
; 14 queens   365,596 solutions in    75,016,170 µs real  1m 16.358s
; 15 queens 2,279,184 solutions in 1,123,821,653 µs real 18m 45.112s

(def N 9)
(def MAX_PRINT 1)

(defn attacks [q1_row q1_col q2_row q2_col] (or
  (= q1_row            q2_row)
  (= (+ q1_col q1_row) (+ q2_col q2_row))
  (= (- q1_col q1_row) (- q2_col q2_row))))

(defn check [queen_rows new_col new_row]
  (loop [i 0]
    (if (< i new_col)
      (if (attacks (queen_rows i) i new_row new_col)
        false
        (recur (inc i)))
      true)))

(defn print_square [queen, white]
    (let [piece (if queen "Q" " ")
          color (if white "1;30;107" "1;97;40")]
        (str "\u001b[" color "m " piece " \u001b[0m")))

(defn print_board [board]
    (apply str
      (for [column (range N)]
        (str
           (apply str (for  [row (range N)]
             (print_square
               (= (board row) column)
               (= (mod (+ column row) 2) 0))))
        "\u001b[0m\n"))))

(defn print_solution [queen_rows new_queen_column new_queen_row output]
  (print (str
    "Solution "
    output "\n"
    (print_board (assoc queen_rows new_queen_column new_queen_row)))))

(defn nine_queens [new_queen_column queen_rows output]
  (loop [new_queen_row 0
         output        output]
    (if (check queen_rows new_queen_column new_queen_row)
      (if (< new_queen_column (dec N)) ; solution valid
        (if (< new_queen_row N)
          (recur
            (inc new_queen_row)
            (nine_queens
              (inc new_queen_column)
              (assoc queen_rows new_queen_column new_queen_row)
              output))
          output)
        (do
            (if (< output MAX_PRINT)
              (print_solution
                queen_rows
                new_queen_column
                new_queen_row
                output))
            (inc output)))
      (if (< new_queen_row (dec N))  ; solution invalid
        (recur (inc new_queen_row) output)
        output))))

(defn main []
  (let [tic (System/nanoTime)
        output (nine_queens 0 (vec (repeat N 0)) 0)
        toc (System/nanoTime)
        elapsed (quot (- toc tic) 1000)]
    (print (str
       N " queens " output " solutions in " elapsed " µs\n"))))

(main)
