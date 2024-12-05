! Copyright (C) 2024 Your name.
! See https://factorcode.org/license.txt for BSD license.
USING: kernel math math.parser prettyprint sequences splitting strings ;
IN: 2024.day-01

CONSTANT: test-input "3   4
4   3
2   5
1   3
3   9
3   3"

: numbers>split ( seq -- seq seq ) dup [ [ first ] map ] dip [ second ] map ;

: listofpairs>pairoflists ( string -- seq seq ) split-lines [ "   " split-subseq ] map numbers>split [ [ >string ] map ] bi@ ;

: pairoflists>distance ( seq seq -- seq ) [ [ string>number ] dip string>number - abs ] 2map sum ;

: part1 ( -- ) test-input listofpairs>pairoflists pairoflists>distance pprint ;

MAIN: part1
