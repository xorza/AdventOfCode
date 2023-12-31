use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
    score: u32,
}

fn parse_card(line: &str) -> Card {
    let mut parts = line.split(": ");
    let id = parts.next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
    let mut all_numbers = parts.next().unwrap().split("|");
    let numbers = all_numbers.nth(0).unwrap().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
    let winning_numbers = all_numbers.nth(0).unwrap().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect::<HashSet<u32>>();

    let score = numbers
        .iter()
        .filter(|x| winning_numbers.contains(x))
        .count() as u32;

    let card = Card { id, numbers, winning_numbers, score };

    // println!("{:?}", card);

    card
}

fn solve1(input: &str) -> u32 {
    input.lines()
        .map(parse_card)
        .map(|card| (1 << card.score) >> 1)
        .sum()
}

fn solve2(input: &str) -> u32 {
    let cards = input.lines()
        .map(parse_card)
        .collect::<Vec<Card>>();

    let mut card_count: Vec<u32> = vec![1; cards.len()];
    cards
        .iter()
        .enumerate()
        .for_each(|(card_index, card)| {
            for next_card_index in card_index + 1..card_index + 1 + card.score as usize {
                card_count[next_card_index] += card_count[card_index];
            }
        });

    card_count.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(INPUT), 13);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(INPUT), 30);
    }

    #[test]
    fn test_input1() {
        const INPUT: &str = include_str!("../inputs/day4.txt");
        let result = solve1(INPUT);
        assert_eq!(result, 20667);
        println!("Result: {}", result);
    }

    #[test]
    fn test_input2() {
        const INPUT: &str = include_str!("../inputs/day4.txt");
        let result = solve2(INPUT);
        assert_eq!(result, 5833065);
        println!("Result: {}", result);
    }
}
