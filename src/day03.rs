use std::collections::HashSet;

#[derive(Debug)]
struct Number {
    value: u32,
    row: isize,
    start: isize,
    end: isize,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    row: isize,
    pos: isize,
}

pub fn main() {
    let input = include_str!("../input/day03.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let input = input.trim_end();
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);
    numbers
        .iter()
        .filter(|n| is_adjacent_to_symbol(n, &symbols))
        .map(|n| n.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    let input = input.trim_end();
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);
    symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| calc_gear_ratio(s, &numbers))
        .sum()
}

fn parse_numbers(input: &str) -> Vec<Number> {
    fn go(
        remaining_row: &str,
        row_number: usize,
        mut acc: Vec<Number>,
        index: usize,
    ) -> Vec<Number> {
        if remaining_row.is_empty() {
            acc
        } else {
            let current_char = remaining_row.chars().next().unwrap();
            if !current_char.is_ascii_digit() {
                go(&remaining_row[1..], row_number, acc, index + 1)
            } else {
                let length = remaining_row
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .count();
                let value = remaining_row[..length].parse::<u32>().unwrap();
                let n = Number {
                    value,
                    row: row_number.try_into().unwrap(),
                    start: index.try_into().unwrap(),
                    end: (index + length - 1).try_into().unwrap(),
                };
                acc.push(n);
                go(&remaining_row[length..], row_number, acc, index + length)
            }
        }
    }

    input
        .split('\n')
        .enumerate()
        .flat_map(|(row_number, row_content)| go(row_content, row_number, Vec::new(), 0))
        .collect::<Vec<_>>()
}

fn parse_symbols(input: &str) -> Vec<Symbol> {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(row_number, row_content)| {
            row_content.chars().enumerate().filter_map(move |(pos, c)| {
                if !c.is_ascii_digit() && c != '.' {
                    Some(Symbol {
                        value: c,
                        row: row_number.try_into().unwrap(),
                        pos: pos.try_into().unwrap(),
                    })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
}

fn is_adjacent_to_symbol(n: &Number, symbols: &[Symbol]) -> bool {
    let adjacent_positions = adjacent_positions(n);
    let positions = symbols
        .iter()
        .map(|s| (s.row, s.pos))
        .collect::<HashSet<_>>();
    let adjacent_symbols = adjacent_positions
        .intersection(&positions)
        .collect::<Vec<_>>();
    !adjacent_symbols.is_empty()
}

fn adjacent_positions(n: &Number) -> HashSet<(isize, isize)> {
    let mut adjacent_positions = (n.start - 1..=n.end + 1)
        .flat_map(|p| vec![(n.row - 1, p), (n.row + 1, p)])
        .collect::<Vec<_>>();
    adjacent_positions.push((n.row, n.start - 1));
    adjacent_positions.push((n.row, n.end + 1));
    HashSet::<(isize, isize)>::from_iter(adjacent_positions)
}

fn calc_gear_ratio(s: &Symbol, numbers: &[Number]) -> u32 {
    let adjacent_numbers = numbers
        .iter()
        .map(|n| (n, adjacent_positions(n)))
        .filter_map(|(n, positions)| {
            if positions.contains(&(s.row, s.pos)) {
                Some(n.value)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if adjacent_numbers.len() == 2 {
        adjacent_numbers
            .iter()
            .copied()
            .reduce(|n1, n2| n1 * n2)
            .unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(part2(input), 467835);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day03.txt");
        assert_eq!(part1(input), 530849);
        assert_eq!(part2(input), 84900879);
    }
}
