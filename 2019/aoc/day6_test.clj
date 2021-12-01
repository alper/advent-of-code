(ns aoc.day6-test
  (:require [clojure.test :refer :all]
            [aoc.day6 :refer :all]))

(deftest adjacency-list-test
  (testing "Testing parsing an adjacency list"
    (is (= [["A" "B"]] (adjacency-list "A)B")))
    (is (= [["A" "B"] ["B" "C"]] (adjacency-list "A)B\nB)C")))))

(deftest tree-descendants-test
  (testing "Testing descendents"
    (is (= ["F" "J"] (tree-descendants (adjacency-list sample) "E")))))

(deftest part1-test
  (testing "Result of part1"
    (is (= 227612 (do-part1)))))