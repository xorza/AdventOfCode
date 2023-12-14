fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = find_digits(line);
            let first_digit = digits.iter().min_by_key(|&(_, line_index)| line_index).unwrap().0;
            let last_digit = digits.iter().max_by_key(|&(_, line_index)| line_index).unwrap().0;
            first_digit * 10 + last_digit
        })
        .sum()
}
const DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_digits(line: &str) -> Vec<(u32, usize)> {
    DIGITS
        .iter()
        .enumerate()
        .flat_map(move |(digit_index, &digit)| {
            line
                .match_indices(digit)
                .map(move |(line_index, _)| (digit_index as u32, line_index))
        })
        .chain(
            line
                .chars()
                .enumerate()
                .filter_map(|(line_index, c)| {
                    c
                        .to_digit(10)
                        .and_then(|digit| Some((digit, line_index)))
                })
        )
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        const TEST_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = solve(TEST_INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn it_works1() {
        const TEST_INPUT: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = solve(TEST_INPUT);
        assert_eq!(result, 281);
    }

    #[test]
    fn it_works1_1() {
        const TEST_INPUT: &str = "vggvnhqkjseventwo4onetwonftrnd";
        let result = solve(TEST_INPUT);
        assert_eq!(result, 72);
    }


    #[test]
    fn input_test() {
        const INPUT: &str = include_str!("../inputs/day1.txt");
        let result = solve(INPUT);
        assert_eq!(result, 54203);
    }
}

