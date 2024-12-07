USING: io io.files io.encodings.utf8 kernel math math.parser regexp prettyprint sequences sequences.extras sorting splitting strings ;
IN: day-03

CONSTANT: test-input "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

: real-input  ( -- string ) "vocab:day-03/input.txt" utf8 file-contents ;

: extract-numbers ( string -- seq ) dup length 1 - 4 swap rot subseq "," split [ string>number ] map ;

: part-1 ( -- x ) real-input R/ mul\(\d+,\d+\)/i all-matching-subseqs [ extract-numbers ] map [ product ] map sum ;

MAIN: [ part-1 . ]