use std::collections::HashMap;

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

pub fn main() {
    let input = include_str!("../input/day02.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .map(parse_game)
        .filter_map(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
                .then_some(game.id)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .map(parse_game)
        .map(|game| {
            let set_of_cubes = minimum_set(game);
            power_of_set(set_of_cubes)
        })
        .sum()
}

fn minimum_set(game: Game) -> Set {
    let red = game.sets.iter().map(|set| set.red).max().unwrap_or(0);
    let green = game.sets.iter().map(|set| set.green).max().unwrap_or(0);
    let blue = game.sets.iter().map(|set| set.blue).max().unwrap_or(0);
    Set { red, green, blue }
}

fn power_of_set(set: Set) -> u32 {
    set.red * set.green * set.blue
}

fn parse_game(encoded_game: &str) -> Game {
    let splitted_game = encoded_game.splitn(2, ':').collect::<Vec<&str>>();
    let header = splitted_game.first().unwrap();
    let extractions = splitted_game.last().unwrap();
    let id = header.replace("Game ", "").parse::<u32>().unwrap();
    let sets = extractions.split(';').map(parse_set).collect();
    Game { id, sets }
}

fn parse_set(encoded_set: &str) -> Set {
    let cubes_extracted = encoded_set
        .split(',')
        .map(|x| x.trim())
        .map(parse_cube)
        .collect::<HashMap<&str, u32>>();
    Set {
        red: *(cubes_extracted.get("red").unwrap_or(&0)),
        green: *(cubes_extracted.get("green").unwrap_or(&0)),
        blue: *(cubes_extracted.get("blue").unwrap_or(&0)),
    }
}

fn parse_cube(encoded_cube_extraction: &str) -> (&str, u32) {
    let splitted_extraction = encoded_cube_extraction
        .splitn(2, ' ')
        .collect::<Vec<&str>>();
    let quantity = splitted_extraction.first().unwrap();
    let color = splitted_extraction.last().unwrap();
    (color, quantity.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day02.txt");
        assert_eq!(part1(input), 2617);
        assert_eq!(part2(input), 59795);
    }
}
