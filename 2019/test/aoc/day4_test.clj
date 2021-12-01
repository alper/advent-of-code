(ns aoc.day4-test
  (:require [clojure.test :refer :all]
            [aoc.day4 :refer :all]))

(deftest digits-test
  (testing "Testing the digits"
    (is (= [1 2 3] (digits 123)))
    ))

(deftest has-independent-double-test
  (testing "Has independent double."
    (is (has-independent-double? 12344))
    (is (not (has-independent-double? 12333)))))

(deftest check-part2-test
  (testing "Testing part 2"
    (is (check-part2? 112233))
    (is (not (check-part2? 123444)))
    (is (check-part2? 111122))))

(deftest check-calc2-test
  (testing "Calculate part 2"
    (is (= (calculateResultPart2) 677))))