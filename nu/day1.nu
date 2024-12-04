
# let data = (open inputs/day1.txt | detect columns --no-headers)


def part1 [input: string] {
    let data = $input
        | detect columns --no-headers
        | each {|x| {a: ($x.column0 | into int), b: ($x.column1 | into int)}}

    let part1 = (
      ($data.a | sort)
          | zip ($data.b | sort)
          | wrap nums
          | insert 'diff' {|x| ($x.nums.0 - $x.nums.1) | math abs }
          | get diff
          | math sum
    )
    return $part1
}

def part2 [input: string] {
    let data = $input
        | detect columns --no-headers
        | each {|x| {a: ($x.column0 | into int), b: ($x.column1 | into int)}}


    let lookup = (
        $data
            | get b
            | histogram
            | reduce --fold {} {|it, acc| {$it.value: $it.count} | merge $acc }
    )

    let result = (
        $data
            | select a
            | insert freq {|x|
                let val = $lookup | get -i ($x.a | into string)

                if ($val | is-empty) {
                    return 0
                }
                return $val
            }
            | each {|x| $x.a * $x.freq}
            | math sum
    )

    return $result
}

# $part1

# TODO part 2

# let freq = ($data | histogram column1)

# $data | select column0 | insert freq {$freq | where column1 == $in.column0 | get count.0? | default 0}
# $data | select column0 | insert freq {$freq | where column1 == '9716' | get count.0? | default 0}
