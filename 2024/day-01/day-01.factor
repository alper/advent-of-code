! Copyright (C) 2024 Your name.
! See https://factorcode.org/license.txt for BSD license.
USING: io.files io.encodings.utf8 kernel math math.parser prettyprint sequences sorting splitting strings ;
IN: 2024.day-01

CONSTANT: test-input "3   4
4   3
2   5
1   3
3   9
3   3"

: real-input  ( -- string ) "vocab:day-01/input.txt" utf8 file-contents ;

: numbers>split ( seq -- seq seq ) dup [ [ first ] map ] dip [ second ] map ;

: listofpairs>pairoflists ( string -- seq seq ) split-lines
    [ "   " split-subseq ] map
    numbers>split [ [ >string string>number ] map sort ] bi@ ;

: pairoflists>distance ( seq seq -- seq ) [ - abs ] 2map sum ;

: part1 ( -- ) real-input listofpairs>pairoflists pairoflists>distance pprint ;

: number>score ( seq number -- number ) dup [ [ = ] curry count ] dip * ;

: part2 ( -- ) real-input listofpairs>pairoflists swap [ [ number>score ] curry ] map [ call( seq -- seq ) ] with map sum pprint ;

MAIN: part2
