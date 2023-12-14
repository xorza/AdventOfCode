#[derive(Debug)]
struct Range {
    src_start: u64,
    dst_start: u64,
    len: u64,
}
#[derive(Debug, Default)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}
#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    maps: [Map; 7],
}

fn parse_almanac(input: &str) -> Almanac {
    let mut result = Almanac::default();
    let mut lines = input.lines();

    result.seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    lines.next(); // skip blank line

    for map in result.maps.iter_mut() {
        map.name = lines
            .next()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .to_string();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split_whitespace();
            let dst_start = parts.next().unwrap().parse::<u64>().unwrap();
            let src_start = parts.next().unwrap().parse::<u64>().unwrap();
            let len = parts.next().unwrap().parse::<u64>().unwrap();

            map.ranges.push(Range { src_start, dst_start, len });
        }
    }

    result
}

fn solve1(input: &str) -> u64 {
    let almanac = parse_almanac(input);

    let mut numbers = almanac.seeds.clone();
    for map in almanac.maps.iter() {
        for number in numbers.iter_mut() {
            for range in map.ranges.iter() {
                if *number > range.src_start && *number < range.src_start + range.len {
                    *number = *number - range.src_start + range.dst_start;
                    break;
                }
            }
        }
    }

    numbers.iter().min().unwrap().to_owned()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 35);
    }
    #[test]
    fn test_input1() {
        let input = include_str!("../inputs/day5.txt");

        assert_eq!(solve1(input), 31599214);
    }

    const INPUT: &str = "\
seeds: 79 14 55 13

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
}