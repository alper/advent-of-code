(ns aoc.core)

(require 'clojure.string)
(require 'clojure.set)

(def filename "suspects.csv")

(def vamp-keys [:name :glitter-index])

(defn str->int
  [str]
  (Integer. str))

(def conversions {:name identity
  :glitter-index str->int})

(defn convert
  [vamp-key value]
  ((get conversions vamp-key) value))

(defn parse
  "Convert a CSV into rows of columns"
  [string]
  (map #(clojure.string/split % #",")
  (clojure.string/split string #"\n")))

(defn mapify
  "Returns a seq of maps"
  [rows]
  (map (fn [unmapped-row]
      (reduce (fn [row-map [vamp-key value]]
                (assoc row-map vamp-key (convert vamp-key value)))
                {}
                (map vector vamp-keys unmapped-row)))
        rows))

(defn glitter-filter
  [minimum-glitter records]
  (filter #(>= (:glitter-index %) minimum-glitter) records))

; Advent of Code day 3

(def day3-input "3-input.txt")

(defn parse-dir-dist
  "Turns a string of form L10 into a vector of a string and an integer."
  [s]
  [(subs s 0 1) (str->int (subs s 1))])

(defn loc-from-loc
  "Loc is a hash with :x and :y, step is a vector with a direction string and an integer. Returns a list of locations that have been stepped over."
  [loc [dir len]]
  (case dir
   "R" (map (fn [step] (update loc :x + step)) (range 1 (+ len 1)))
   "L" (map (fn [step] (update loc :x - step)) (range 1 (+ len 1)))
   "U" (map (fn [step] (update loc :y + step)) (range 1 (+ len 1)))
   "D" (map (fn [step] (update loc :y - step)) (range 1 (+ len 1))))
)

(def origin {:x 0 :y 0})

(defn generate-points-from-steps
  "For a vector of steps of the form [\"R\" 10] etc., generates a vector of points starting from origin."
  [steps]
  (reduce 
    (fn [path step] (into path (loc-from-loc (peek path) step)))
    [origin]
    steps))


(defn find-overlaps
  "Takes two lists of points and returns the ones that are in both."
  [points1 points2]
  ; (println points1)
  ; (println points2)
  (remove
   #(= origin %)
   (clojure.set/intersection (set points1) (set points2))))

(defn dist-to-origin
  [point]
  (+ (Math/abs (:x point)) (Math/abs (:y point))))

(defn closest-to-origin
  "For a series of points, orders them and returns the point that is closest to the origin."
  [points]
  (println points)
  (first (sort
          (fn [p1 p2] (< (dist-to-origin p1) (dist-to-origin p2)))
          points)))

(defn parse-day3
  "Parse day 3 input"
  [string]
  (map 
    (fn [vec] (map parse-dir-dist vec))
    (map
      #(clojure.string/split % #",")
      (clojure.string/split string #"\n"))))

(def sample0 "R8,U5,L5,D3\nU7,R6,D4,L4")
(def sample1 "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
(def sample2 "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
(def full-input (slurp "3-input.txt"))

(defn -main
  []
  (println "Running")
  (let [[one two] (parse-day3 full-input)]
    (println "Distance of the shortest overlap is:"(dist-to-origin (closest-to-origin (find-overlaps
                                         (generate-points-from-steps one) (generate-points-from-steps two)))))))