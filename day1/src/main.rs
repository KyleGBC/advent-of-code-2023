use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let nums = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
        let mut num_str = nums.first().unwrap().to_string();
        num_str.push(*nums.last().unwrap());
        let num = num_str.parse::<i32>().unwrap();
        sum += num;
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let digit_words: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for line in input.lines() {
        let mut digit_matches: Vec<(usize, char)> = Vec::new();
        for key in digit_words.keys() {
            digit_matches.extend(
                line.match_indices(key)
                    .map(|(i, s)| (i, *digit_words.get(s).unwrap())),
            );
        }
        digit_matches.extend(
            line.match_indices(|c: char| c.is_digit(10))
                .map(|(i, s)| (i, s.chars().next().unwrap())),
        );
        digit_matches.sort_by(|(i, _), (i2, _)| i.cmp(i2));
        let digit_matches: Vec<char> = digit_matches.into_iter().map(|(_, c)| c).collect();

        let mut num_str = digit_matches.first().unwrap().to_string();
        num_str.push(*digit_matches.last().unwrap());
        let num = num_str.parse::<i32>().unwrap();
        sum += num;
    }
    sum
}

fn main() {
    let input = include_str!("..\\input.txt");
    let part1 = part1(input);
    let part2: i32 = part2(input);

    println!("part1: {part1}, part2: {part2}");
}