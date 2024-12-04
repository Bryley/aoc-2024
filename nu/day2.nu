
def part1 [input: string] {
    let data = (
        $input
            | lines
            | each {$in | split words | into int }
    )

    let result = (
        $data
            | each { {
                inp: ($in | str join ' '),
                res: ($in | is_valid_part1)
            } }
            | filter {$in.res}
            | length
    )

    return $result
}

def part2 [input: string] {
    let data = $input
        | lines
        | each {$in | split words | into int }

    let result = $data
        | each { {
            inp: ($in | str join ' '),
            res: ($in | is_valid_part2)
        } }
        | filter {$in.res}
        | length

    return $result
}

def is_valid_part1 [] {
    let diffs = $in
        | zip ($in | skip 1)
        | each {|x| $x.1 - $x.0}

    let check1 = $diffs
        | filter {|x| ($x | math abs) > 3}
        | is-empty
    let is_postive = $diffs | all {|x| $x > 0}
    let is_negative = $diffs | all {|x| $x < 0}

    return ($check1 and ($is_postive or $is_negative))
}

def is_valid_part2 [] {
    if ($in | is_valid_part1) {
        return true
    }

    let data = $in | enumerate

    # Check each permutation of removed element of the result using part1
    let permutations = (
        $data
            | each {|x| $data | where index != $x.index | get item }
    )

    return (
        $permutations
            | each {|x| $x | is_valid_part1}
            | any {}
    )
}
