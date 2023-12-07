struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn convert(&self, raw: u64) -> u64 {
        self.conversions
            .iter()
            .find_map(|conversion| conversion.convert(raw))
            .unwrap_or(raw)
    }

    fn convert_range(&self, range: Range) -> Vec<Range> {
        let mut converted_ranges = vec![];
        let mut remaining = vec![range];
        for conversion in self.conversions.iter().as_slice() {
            let (mut converted, new_remaining): (Vec<Range>, Vec<Range>) = remaining.iter().fold(
                (vec![], vec![]),
                |(mut converted_acc, mut remaining_acc), r| {
                    let (converted, mut outbounds) = conversion.convert_range(r);
                    if let Some(c) = converted {
                        converted_acc.push(c);
                    }

                    remaining_acc.append(&mut outbounds);

                    (converted_acc, remaining_acc)
                },
            );

            converted_ranges.append(&mut converted);
            remaining = new_remaining;
        }
        converted_ranges.append(&mut remaining);
        converted_ranges
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    from: u64,
    to: u64,
}

#[derive(Debug)]
struct Conversion {
    source: u64,
    destination: u64,
    range: u64,
}

impl Conversion {
    fn convert(&self, n: u64) -> Option<u64> {
        if self.source <= n && n < self.source + self.range {
            Some(self.destination + n - self.source)
        } else {
            None
        }
    }

    fn convert_range(&self, range: &Range) -> (Option<Range>, Vec<Range>) {
        if self.contains(range) {
            let from = self.convert(range.from).unwrap();
            let to = self.convert(range.to).unwrap();
            (Some(Range { from, to }), vec![])
        } else if self.is_contained(range) {
            let converted_range = Range {
                from: self.destination,
                to: self.destination + self.range,
            };
            let outbound_left_range = Range {
                to: self.source,
                ..*range
            };
            let outbound_right_range = Range {
                from: self.source + self.range,
                ..*range
            };
            (
                Some(converted_range),
                vec![outbound_left_range, outbound_right_range],
            )
        } else if self.overlaps_left(range) {
            let converted_range = Range {
                from: self.destination,
                to: self.convert(range.to).unwrap(),
            };
            let outbound_range = Range {
                to: self.source,
                ..*range
            };
            (Some(converted_range), vec![outbound_range])
        } else if self.overlaps_right(range) {
            let converted_range = Range {
                from: self.convert(range.from).unwrap(),
                to: self.destination + self.range,
            };
            let outbound_range = Range {
                from: self.source + self.range,
                ..*range
            };
            (Some(converted_range), vec![outbound_range])
        } else {
            (None, vec![*range])
        }
    }

    //    cccccccccc
    //       rrrr
    fn contains(&self, range: &Range) -> bool {
        self.is_inside(range.from) && self.is_inside(range.to)
    }

    //      cccccccc
    //  rrrrrrrrrrrrrr
    fn is_contained(&self, range: &Range) -> bool {
        self.is_outside_left(range.from) && self.is_outside_right(range.to)
    }

    //      cccccccc
    //  rrrrrrr
    fn overlaps_left(&self, range: &Range) -> bool {
        self.is_outside_left(range.from) && self.is_inside(range.to)
    }

    //      cccccccc
    //          rrrrrrr
    fn overlaps_right(&self, range: &Range) -> bool {
        self.is_inside(range.from) && self.is_outside_right(range.to)
    }

    fn is_inside(&self, v: u64) -> bool {
        self.source <= v && v < self.source + self.range
    }

    fn is_outside_left(&self, v: u64) -> bool {
        v < self.source
    }

    fn is_outside_right(&self, v: u64) -> bool {
        self.source + self.range <= v
    }
}

pub fn main() {
    let input = include_str!("../input/day05.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    seeds
        .iter()
        .map(|&seed| {
            maps.iter()
                .fold(seed, |current_state, map| map.convert(current_state))
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let (ranged_seeds, maps) = parse_input2(input);
    ranged_seeds
        .iter()
        .flat_map(|&range| {
            maps.iter().fold(vec![range], |ranges, map| {
                ranges.iter().flat_map(|r| map.convert_range(*r)).collect()
            })
        })
        .map(|r| r.from)
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let (raw_seeds, raw_maps) = input.trim().split_once("\n\n").unwrap();
    (parse_seeds(raw_seeds), parse_maps(raw_maps))
}

fn parse_input2(input: &str) -> (Vec<Range>, Vec<Map>) {
    let (raw_seeds, raw_maps) = input.trim().split_once("\n\n").unwrap();
    (parse_seeds2(raw_seeds), parse_maps(raw_maps))
}

fn parse_seeds(raw_seeds: &str) -> Vec<u64> {
    raw_seeds
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|raw_n| raw_n.parse().unwrap())
        .collect()
}

fn parse_seeds2(raw_seeds: &str) -> Vec<Range> {
    let raw_seeds = raw_seeds.replace("seeds: ", "");
    let raw_numbers = raw_seeds.split_whitespace().collect::<Vec<_>>();

    raw_numbers
        .chunks(2)
        .map(|l| {
            let from = l[0].parse::<u64>().unwrap();
            let length = l[1].parse::<u64>().unwrap();
            Range {
                from,
                to: from + length,
            }
        })
        .collect::<Vec<_>>()
}

fn parse_maps(raw_maps: &str) -> Vec<Map> {
    raw_maps.split("\n\n").map(parse_map).collect()
}

fn parse_map(raw_map: &str) -> Map {
    let (_header, maps) = raw_map.split_once('\n').unwrap();
    let conversions = maps.split('\n').map(parse_conversion).collect();
    Map { conversions }
}

fn parse_conversion(raw_conversion: &str) -> Conversion {
    let splitted_conversion = raw_conversion.split_whitespace().collect::<Vec<_>>();
    Conversion {
        source: splitted_conversion[1].parse().unwrap(),
        destination: splitted_conversion[0].parse().unwrap(),
        range: splitted_conversion[2].parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part2(input), 46);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day05.txt");
        assert_eq!(part1(input), 199602917);
        assert_eq!(part2(input), 2254686);
    }
}
