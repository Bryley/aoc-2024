
let data = open ./inputs/input2.txt | lines | each {$in | split words | into int }

def is_valid_part1 [] {
    let diffs = $in | zip ($in | skip 1) | each {|x| $x.1 - $x.0}

    let check1 = $diffs | filter {|x| ($x | math abs) > 3} | is-empty
    let is_postive = $diffs | all {|x| $x > 0}
    let is_negative = $diffs | all {|x| $x < 0}

    $check1 and ($is_postive or $is_negative)
}

def is_valid_part2 [] {
    let diffs = $in | zip ($in | skip 1) | each {|x| $x.1 - $x.0}

    let positive_diffs = $diffs | enumerate | filter {
        $in.index != (
            $diffs | enumerate | filter {$in.item <= 0 or $in.item > 3} | get index.0?
        )
    } | get item

    let negative_diffs = $diffs | enumerate | filter {
        $in.index != (
            $diffs | enumerate | filter {$in.item >= 0 or $in.item < -3} | get index.0?
        )
    } | get item

    let positive_diffs_check1 = $positive_diffs | filter {|x| ($x | math abs) > 3} | is-empty
    let positive_diffs_is_postive = $positive_diffs | all {|x| $x > 0}
    let positive_diffs_is_negative = $positive_diffs | all {|x| $x < 0}

    let negative_diffs_check1 = $negative_diffs | filter {|x| ($x | math abs) > 3} | is-empty
    let negative_diffs_is_postive = $negative_diffs | all {|x| $x > 0}
    let negative_diffs_is_negative = $negative_diffs | all {|x| $x < 0}

    (
        ($positive_diffs_check1 and ($positive_diffs_is_postive or $positive_diffs_is_negative))
        or
        ($negative_diffs_check1 and ($negative_diffs_is_postive or $negative_diffs_is_negative))
    )
}

let part1 = $data | each { {
    inp: ($in | str join ' '),
    res: ($in | is_valid_part1)
} } | filter {$in.res} | length

let part2 = $data | each { {
    inp: ($in | str join ' '),
    res: ($in | is_valid_part2)
} } | filter {$in.res} | length

printf "Part1: %s | Part2: %s\n" $part1 $part2
