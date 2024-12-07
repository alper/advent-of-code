USING: io io.files io.encodings.utf8 kernel math math.parser prettyprint sequences sequences.extras sorting splitting strings ;
IN: day-02

CONSTANT: test-input "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"

: real-input  ( -- string ) "vocab:day-02/input.txt" utf8 file-contents ;

: all-inc/dec ( seq -- ? ) dup 1 rotate [ - ] 2map but-last [ [ 0 > ] all? ] [ [ 0 < ] all? ] bi or ;

: all-within ( seq -- ? ) dup 1 rotate [ - abs ] 2map but-last [ [ 0 > ] all? ] [ [ 4 < ] all? ] bi and ;

: part-1 ( -- x ) real-input split-lines [ " " split-subseq ] map [ [ string>number ] map ] map [ [ all-inc/dec ] [ all-within ] bi and ] map sift length ;

MAIN: [ "test" pprint ]