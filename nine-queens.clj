;; time java -jar ../clojure/clojure.jar nine-queens.clj

;  0 queens          1 solutions in         505 µs real  0m  1.193s
;  1 queens          1 solutions in       1,090 µs real  0m  1.184s
;  2 queens          0 solutions in         280 µs real  0m  1.266s
;  3 queens          0 solutions in         299 µs real  0m  1.225s
;  4 queens          2 solutions in       2,932 µs real  0m  1.371s
;  5 queens         10 solutions in       3,111 µs real  0m  1.304s
;  6 queens          4 solutions in       3,778 µs real  0m  1.202s
;  7 queens         40 solutions in       4,807 µs real  0m  1.179s
;  8 queens         92 solutions in      10,531 µs real  0m  1.212s
;  9 queens        352 solutions in      26,353 µs real  0m  1.265s
; 10 queens        724 solutions in      42,311 µs real  0m  1.265s
; 11 queens      2,680 solutions in     108,141 µs real  0m  1.295s
; 12 queens     14,200 solutions in     434,473 µs real  0m  1.626s
; 13 queens     73,712 solutions in   2,443,799 µs real  0m  3.621s
; 14 queens    365,596 solutions in  16,377,622 µs real  0m 17.581s
; 15 queens  2,279,184 solutions in 104,185,393 µs real  1m 45.414s
; 16 queens 14,772,512 solutions in 812,623,958 µs real 13m 33.912s

(set! *unchecked-math* true)

(def ^:const N         9)
(def ^:const MAX_PRINT 1)

(defn attacks [^long q1_row ^long q1_col ^long q2_row ^long q2_col] (or
  (= q1_row            q2_row)
  (= (+ q1_col q1_row) (+ q2_col q2_row))
  (= (- q1_col q1_row) (- q2_col q2_row))))

(defn check [queen_rows ^long new_col ^long new_row]
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

(defn nine_queens [^long new_queen_column queen_rows ^long output]
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
            (print_solution queen_rows new_queen_column new_queen_row output))
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
