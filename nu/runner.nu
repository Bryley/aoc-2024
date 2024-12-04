
def main [day: int] {
    if $day == 1 {
        source "./day1.nu"

        mut part1_result = null
        mut part2_result = null
        let part1_duration = timeit { $part1_result = (part1 (open $"inputs/day($day).txt")) }
        let part2_duration = timeit { $part2_result = (part2 (open $"inputs/day($day).txt")) }

        output $day $part1_result $part2_result $part1_duration $part2_duration
    } else if $day == 2 {
        source "./day2.nu"

        mut part1_result = null
        mut part2_result = null
        let part1_duration = timeit { $part1_result = (part1 (open $"inputs/day($day).txt")) }
        let part2_duration = timeit { $part2_result = (part2 (open $"inputs/day($day).txt")) }

        output $day $part1_result $part2_result $part1_duration $part2_duration
    } else if $day == 3 {
        source "./day3.nu"

        mut part1_result = null
        mut part2_result = null
        let part1_duration = timeit { $part1_result = (part1 (open $"inputs/day($day).txt")) }
        let part2_duration = timeit { $part2_result = (part2 (open $"inputs/day($day).txt")) }

        output $day $part1_result $part2_result $part1_duration $part2_duration
    } else if $day == 4 {
        source "./day4.nu"

        mut part1_result = null
        mut part2_result = null
        let part1_duration = timeit { $part1_result = (part1 (open $"inputs/day($day).txt")) }
        let part2_duration = timeit { $part2_result = (part2 (open $"inputs/day($day).txt")) }

        output $day $part1_result $part2_result $part1_duration $part2_duration
    } else {
        print $"Day ($day) not found"
    }
}

def output [day: int, part1, part2, part1_dur: duration, part2_dur: duration] {

    print $"Nushell Day ($day) results:"
    print $"  Part1: ($part1) in ($part1_dur | format duration ms)"
    print $"  Part2: ($part2) in ($part2_dur | format duration ms)"
}
