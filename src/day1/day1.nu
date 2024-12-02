
let data = (open inputs/input1.txt | detect columns --no-headers)

let part1 = (
  ($data.column0 | sort) | zip ($data.column1 | sort) | each {|x| {a: ($x.0 | into int), b: ($x.1 | into int)}} | insert 'diff' { ($in.a - $in.b) | math abs } | get diff | math sum
)

$part1

# TODO

# let freq = ($data | histogram column1)

# $data | select column0 | insert freq {$freq | where column1 == $in.column0 | get count.0? | default 0}
# $data | select column0 | insert freq {$freq | where column1 == '9716' | get count.0? | default 0}
