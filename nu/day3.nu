let data = open ./inputs/day3.txt

let part1 = $data | parse --regex 'mul\((\d{1,3}),(\d{1,3})\)' | each {|x| (($x.capture0 | into int) * ($x.capture1 | into int)) } | math sum

let captures = $data | parse --regex "mul\\((\\d{1,3}),(\\d{1,3})\\)|(do\\(\\)|don't\\(\\))" | rename num1 num2 do

let new_do = $captures | get do | reduce --fold ["do()"] {
    |it,acc| $acc | append ( if ($it | is-empty) { $acc | reverse | take 1 } else { $it } )
} | skip 1 | wrap do

let part2 = $captures | merge $new_do | filter {|x| $x.num1 | is-not-empty} | where do != "don't()" | each {|x| ($x.num1 | into int) * ($x.num2 | into int)} | math sum

printf "Part1: %s | Part2: %s\n" $part1 $part2
