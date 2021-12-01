(ns aoc.day8
  (:require [clojure.string :as s]
            [clojure.set]))

; Part 1

(def sample-data "123456789012")

(def width 25)
(def height 6)

(defn number-of
  [c s]
  (count (filter #(= % c) s)))

(defn result-mult
  [s]
  (* 
   (number-of \1 s) 
   (number-of \2 s)))

(defn do-part1
  []
  (->> (slurp "8-input.txt")
       (partition (* width height))
       (sort #(compare (number-of \0 %1) (number-of \0 %2)))
       (first)
       (result-mult)))

; => 1072

; part 2

(def sample2 "0222112222120000")
(def sample-width 2)
(def sample-height 2)

(defn calculate-color
  [pxs]
  (->> pxs
       (drop-while #(= 2 %))
       (first)))

(defn do-part2
  []
  (let [data (slurp "8-input.txt")
        width width
        pixels (* width height)]
    (->> data
         (map (comp #(Integer/parseInt %) str))
         (partition pixels)
         (apply interleave)
         (partition (/ (count data) pixels))
         (map calculate-color)
         (partition width))))

; ((1 0 0 0 1 1 0 0 0 0 1 1 1 1 0 1 1 1 0 0 0 0 1 1 0)
;  (1 0 0 0 1 1 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 1 0)
;  (0 1 0 1 0 1 0 0 0 0 1 1 1 0 0 1 0 0 1 0 0 0 0 1 0)
;  (0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 0 1 0)
;  (0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 1 0)
;  (0 0 1 0 0 1 1 1 1 0 1 0 0 0 0 1 0 0 0 0 0 1 1 0 0))

; YLFPJ