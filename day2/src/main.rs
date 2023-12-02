use regex::Regex;
use once_cell::sync::Lazy;

static CUBE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d+) (?P<color>green|red|blue)(?:,|;)?").expect("Invalid regular expression"));
static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?P<id>\d+):").expect("Invalid regular expression"));

fn part1(input: &str, maxes: [u32; 3]) -> u32
{
    let mut sum = 0;
    for line in input.lines()
    {
        let mut possible = true;
        let id = ID_REGEX.captures(line).unwrap().name("id").expect("no id number on this line").as_str().parse::<u32>().expect("id couldn't be parsed to a number"); 
        
        for caps in CUBE_REGEX.captures_iter(line)
        {
            let (_, [amount, color]) = caps.extract(); 
            let amount = amount.parse::<u32>().expect("Invalid amount value, couldn't be parsed to a number");
            let max_idx = match color { "red" => 0, "green" => 1, "blue" => 2, _ => unreachable!(), };
            if amount > maxes[max_idx]
            { 
                possible = false;
                break;
            }
        }
        if possible { sum += id };
    }
    sum
}

fn part2(input: &str) -> u32 
{
    let mut sum = 0;
    for line in input.lines()
    {
        let mut mins = [0, 0, 0];
        
        for caps in CUBE_REGEX.captures_iter(line)
        {
            let (_, [amount, color]) = caps.extract(); 
            let amount = amount.parse::<u32>().expect("Invalid amount value, couldn't be parsed to a number");
            let min_idx = match color { "red" => 0, "green" => 1, "blue" => 2, _ => unreachable!(), };
            if amount > mins[min_idx]
            { 
                mins[min_idx] = amount;
            }
        }
        sum += mins[0] * mins[1] * mins[2]
    }
    sum
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let part1 = part1(input, [12, 13, 14]);
    let part2 = part2(input);
    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", start.elapsed());
}
