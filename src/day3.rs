struct Number {
    value: u32,
    line_index: usize,
    number_start: usize,
    number_end: usize,
}

fn is_symbol(c: char) -> bool {
    c != '.'
}

fn is_a_part(lines: &[&str], line_index: usize, number_start: usize, number_end: usize) -> bool {
    let line = lines[line_index];
    if number_start > 0 && is_symbol(line.chars().nth(number_start - 1).unwrap()) {
        return true;
    }
    if number_end < line.len() - 1 && is_symbol(line.chars().nth(number_end).unwrap()) {
        return true;
    }

    let range_start = number_start.saturating_sub(1);
    let range_end = (number_end + 1).min(line.len());

    if line_index > 0 && lines[line_index - 1][range_start..range_end].chars().any(is_symbol) {
        return true;
    }
    if line_index < lines.len() - 1 && lines[line_index + 1][range_start..range_end].chars().any(is_symbol) {
        return true;
    }

    false
}

fn find_numbers(lines: &Vec<&str>) -> Vec<Number> {
    let mut result: Vec<Number> = Vec::new();

    for line_index in 0..lines.len() {
        let line = lines[line_index];
        let mut carret: usize = 0;

        while carret < line.len() {
            if let Some(number_start) = line[carret..].find(|c: char| c.is_numeric()) {
                let number_start = carret + number_start;
                let number_length = line[number_start..]
                    .find(|c: char| !c.is_numeric())
                    .unwrap_or(line.len() - number_start);
                let number_end = number_start + number_length;

                let value = line[number_start..number_end].parse::<u32>().unwrap();
                let number = Number {
                    value,
                    line_index,
                    number_start,
                    number_end,
                };

                result.push(number);

                // println!("{} {}", number, is_a_part);
                // println!("{} {} {}", number_start, number_end, number);

                carret = number_end;
            } else {
                carret = line.len();
            }
        }
    }

    result
}

fn solve1(input: &str) -> u32 {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    find_numbers(&lines)
        .iter()
        .filter(|&number| is_a_part(lines.as_slice(), number.line_index, number.number_start, number.number_end))
        .map(|number| number.value)
        .sum()
}

fn solve2(input: &str) -> u32 {
    let mut result = 0;

    let lines = input.trim().lines().collect::<Vec<&str>>();
    let numbers = find_numbers(&lines);

    for line_index in 0..lines.len() {
        let line = lines[line_index];
        for symbol_index in 0..line.len() {
            if Some('*') == line.chars().nth(symbol_index) {
                let numbers = numbers
                    .iter()
                    .filter(|&number| {
                        line_index >= number.line_index.saturating_sub(1)
                            && line_index <= number.line_index + 1
                            && symbol_index >= number.number_start.saturating_sub(1)
                            && symbol_index < number.number_end + 1
                    })
                    .map(|number| number.value)
                    .collect::<Vec<u32>>();

                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
467..114..
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

    #[test]
    fn test_solve1() {
        let result = solve1(INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_solve2() {
        let result = solve2(INPUT);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input1() {
        const INPUT: &str = include_str!("../inputs/day3.txt");
        let result = solve1(INPUT);
        assert_eq!(result, 528819);
        println!("Result: {}", result);
    }


    #[test]
    fn test_input2() {
        const INPUT: &str = include_str!("../inputs/day3.txt");
        let result = solve2(INPUT);
        assert_eq!(result, 80403602);
        println!("Result: {}", result);
    }
}