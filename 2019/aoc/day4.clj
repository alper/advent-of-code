(ns aoc.day4
;   (:require [clojure.string :as s])
  )

(def lower-bound 246515)
(def upper-bound 739105)

(defn digits [n]
  (->> n
       (iterate #(quot % 10))
       (take-while pos?)
       (mapv #(mod % 10))
       rseq))

(defn has-double? [num] (some? (some true? (map #(= %1 %2) (str num) (rest (str num))))))

(defn has-independent-double? [num] (->>
                                     num
                                     digits
                                     (partition-by identity)
                                     (map count)
                                     (some #(= % 2))
                                     ))

(defn monotone? [num] (every? true? (map
                                    #(<= %1 %2)
                                    (digits num)
                                    (rest (digits num)))))

(defn check? [num] (and (has-double? num) (monotone? num)))

(defn check-part2? [num] (and (has-independent-double? num) (monotone? num)))

(defn calculateResult [] (count (filter check? (range lower-bound upper-bound))))

; Not bothering with part 2

(defn calculateResultPart2 [] (count (filter check-part2? (range lower-bound upper-bound))))

(defn -main
  []
  (println (calculateResult))
  (println (calculateResultPart2)))