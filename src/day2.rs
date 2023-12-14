#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(": ");
    let id = parts.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();

    let rounds = parts
        .next()
        .unwrap()
        .split("; ")
        .map(|round_str| {
            let mut round = Round::default();

            round_str
                .split(", ")
                .for_each(|color_str| {
                    let mut color = color_str.split(" ");
                    let count = color.next().unwrap().parse::<u32>().unwrap();
                    let color = color.next().unwrap();

                    match color {
                        "red" => round.red = count,
                        "green" => round.green = count,
                        "blue" => round.blue = count,
                        _ => panic!("Unknown color {}", color),
                    }
                });

            round
        }).collect::<Vec<Round>>();

    Game { id, rounds }
}

fn solve1(input: &str, max_round: &Round) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = parse_game(line);

            // println!("Line {}", line);
            // println!("Game {:?}", game);
            // println!()

            game.rounds
                .iter()
                .find(|&round| {
                    round.red > max_round.red || round.green > max_round.green || round.blue > max_round.blue
                })
                .map_or_else(|| Some(game.id), |_| None)
        })
        .sum::<u32>()
}

fn solve2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let game = parse_game(line);

            let min_round = game.rounds
                .iter()
                .fold(Round::default(), |mut min_round, round| {
                    min_round.red = min_round.red.max(round.red);
                    min_round.green = min_round.green.max(round.green);
                    min_round.blue = min_round.blue.max(round.blue);

                    min_round
                });

            let power = min_round.red * min_round.green * min_round.blue;
            // println!("Game {} power {}", game.id, power);
            power
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn it_works0() {
        let result = solve1(INPUT, &Round { red: 12, green: 13, blue: 14 });
        assert_eq!(result, 8);
    }
    #[test]
    fn it_works1() {
        let result = solve2(INPUT);
        assert_eq!(result, 2286);
    }

    #[test]
    fn input_test1() {
        const INPUT: &str = include_str!("../inputs/day2.txt");
        let result = solve1(INPUT, &Round { red: 12, green: 13, blue: 14 });
        assert_eq!(result, 2720);
    }

    #[test]
    fn input_test2() {
        const INPUT: &str = include_str!("../inputs/day2.txt");
        let result = solve2(INPUT);
        assert_eq!(result, 71535);
    }
}
