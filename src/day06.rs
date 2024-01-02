struct Race {
    time: u64,
    best_distance: u64,
}

pub fn main() {
    let input = include_str!("../input/day06.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(count_number_of_ways_to_win)
        .reduce(|w1, w2| w1 * w2)
        .unwrap()
}

fn part2(input: &str) -> u64 {
    count_number_of_ways_to_win(&parse_input2(input))
}

fn parse_input(input: &str) -> Vec<Race> {
    let (time_line, distance_line) = input.trim().split_once('\n').unwrap();
    let times = parse_times(time_line);
    let distances = parse_distances(distance_line);
    times
        .iter()
        .zip(distances)
        .map(|(&time, best_distance)| Race {
            time,
            best_distance,
        })
        .collect()
}

fn parse_input2(input: &str) -> Race {
    let (time_line, distance_line) = input.trim().split_once('\n').unwrap();
    let time = time_line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let best_distance = distance_line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    Race {
        time,
        best_distance,
    }
}

fn parse_times(line: &str) -> Vec<u64> {
    line.replace("Time:", "")
        .trim()
        .split(' ')
        .filter(|r| !r.is_empty())
        .map(|t| t.trim().parse().unwrap())
        .collect()
}

fn parse_distances(line: &str) -> Vec<u64> {
    line.replace("Distance:", "")
        .trim()
        .split(' ')
        .filter(|r| !r.is_empty())
        .map(|d| d.trim().parse().unwrap())
        .collect()
}

fn count_number_of_ways_to_win(race: &Race) -> u64 {
    (0..race.time)
        .map(|ms_holding| {
            let speed = ms_holding;
            speed * (race.time - ms_holding)
        })
        .filter(|distance| distance > &race.best_distance)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(part2(input), 71503);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day06.txt");
        assert_eq!(part1(input), 505494);
        assert_eq!(part2(input), 23632299);
    }
}
