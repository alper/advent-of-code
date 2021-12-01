(ns aoc.day6
  (:require [clojure.string :as s]
            [clojure.set]))

; Part 1

(def sample "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L")

(def sample-answer 42)

(defn adjacency-list
  [s]
  (->> s
       (s/split-lines)
       (map #(s/split % #"\)"))))

(defn tree-root
  [adj-list]
  (let [orbiter-set (->> adj-list
                         (map second)
                         (set))
        orbitee-set (->> adj-list
                         (map first)
                         (set))
        root-orbitee (->> orbitee-set
                          (filter #(not (contains? orbiter-set %1))))]
    (first root-orbitee)))

(defn tree-descendants
  [adj-list val]
  (->> adj-list
       (filter #(= val (first %)))
       (map second)))

(defn ->leaf
  [val]
  [val])

(defn ->branch
  [val children]
  [val children])

(defn ->tree
  [adj-list val]
  (let [->tree' (partial ->tree adj-list)
        kids (tree-descendants adj-list val)]
    (if-not (empty? kids)
      (->branch val (map ->tree' kids))
      (->leaf val))))

(defn sum-of-depths
  [branches]
  (loop [branches branches
         cur-depth 0
         total-depth 0]
    (cond
      (empty? branches) total-depth
      :else (recur
             (mapcat (fn [node] (second node)) branches)
             (inc cur-depth)
             (+ total-depth (* (count branches) cur-depth))))))

(defn do-part1
  []
  (let [adjacencies (adjacency-list (slurp "6-input.txt"))
        root (tree-root adjacencies)
        tree (->tree adjacencies root)]
    (sum-of-depths [tree])))

; Part 2

(def sample2 "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN")

(defn put-orbits-in-map
  [coll, pair]
  (update coll 
          (first pair) 
          #(concat %1 [(second pair)])))

(defn adjacency-map
  "Creates a map of the adjacencies which makes building a tree unnecessary."
  [s]
  (->> s
       (s/split-lines)
       (map #(s/split % #"\)"))
       (reduce put-orbits-in-map {})))

(defn get-key-for-value
  "Given a v and a map return the key whose list value contains v."
  [m v]
  (first (keep 
          #(when 
            (not= -1 (.indexOf (val %) v))
             (key %))
          m)))

(defn path-to-root
  "The core idea of this solution is to calculate the path to the root for both nodes. The part of the path that is not common is where both sides branch of and as such those two fragments are the path to get from one to the other."
  [adjacency-map val]
  (loop [child val
         parent-key (get-key-for-value adjacency-map child)
         path []]
    (if parent-key
      (recur parent-key 
             (get-key-for-value adjacency-map parent-key) 
             (conj path parent-key))
      path)))

(defn do-part2
  []
  (let [adj-map (adjacency-map (slurp "6-input.txt"))
        path1 (set (path-to-root adj-map "YOU"))
        path2 (set (path-to-root adj-map "SAN"))
        unique1 (clojure.set/difference path1 path2)
        unique2 (clojure.set/difference path2 path1)]
    (+ (count unique1) (count unique2))))

; => 454