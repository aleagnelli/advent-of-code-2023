pub fn main() {
    let input = include_str!("../input/day01.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .map(recover_calibration_value)
        .sum()
}

fn recover_calibration_value(raw: &str) -> u32 {
    let digits = raw
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn part2(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .map(recover_spelled_calibration_value)
        .sum()
}

fn recover_spelled_calibration_value(raw: &str) -> u32 {
    let spelled_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let (first_digit, _pos) = spelled_digits
        .iter()
        .map(|digit| (digit, raw.find(digit)))
        .filter(|(_d, optional_pos)| optional_pos.is_some())
        .min_by_key(|(_d, pos)| pos.unwrap())
        .unwrap();

    let (last_digit, _pos) = spelled_digits
        .iter()
        .map(|digit| (digit, raw.rfind(digit)))
        .filter(|(_d, optional_pos)| optional_pos.is_some())
        .max_by_key(|(_d, pos)| pos.unwrap())
        .unwrap();

    translate_spelled_digit(first_digit) * 10 + translate_spelled_digit(last_digit)
}

fn translate_spelled_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day01.txt");
        assert_eq!(part1(input), 54561);
        assert_eq!(part2(input), 54076);
    }
}
