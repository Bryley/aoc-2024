

def part1 [input: string] {
    let data = $input
        | lines
        | each {|x| $x | split chars}

    # TODO have get fn for $data, then look from all angles

    let width = $data | get 0 | length
    let height = $data | length

    0..$height | each {|row| 0..$width | each {|col| [$row, $col]} }
    
    return ("Not completed yet")
}

def part2 [input: string] {
}
