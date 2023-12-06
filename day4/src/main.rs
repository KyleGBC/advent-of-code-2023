fn parse_scratchcard(line: &str) -> (Vec<i32>, Vec<i32>) {

    let mut winning_nums: Vec<i32> = Vec::with_capacity(10);
    let mut your_nums: Vec<i32> = Vec::with_capacity(25);
    
    let (_, line) = line.split_once(':').expect("Line contains no :");
    let line = line.trim_start();
    let (winners, yours) = line.split_once('|').expect("Line contains no |");

    winning_nums.extend(winners.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()));
    your_nums.extend(yours.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()));

    (winning_nums, your_nums)
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let cards = input.lines().map(|l| parse_scratchcard(l)).collect::<Vec<_>>();

    let mut copies_and_matches: Vec<(u32, u32)> = cards.into_iter().map(|(w, y)| y.iter().map(|n| w.contains(n)).filter(|b| *b).count() as u32).map(|c| (1, c)).collect();
    let part1: u32 = copies_and_matches.iter().map(|(_, v)| if *v == 0 { 0 } else { 2u32.pow(v.saturating_sub(1)) } ).sum();

    for idx in 0..copies_and_matches.len()
    {
        let (copies, num_matches) = copies_and_matches[idx];
        for dist in 1..=(num_matches as usize)
        {
            copies_and_matches[idx + dist].0 += copies;
        }
    }
    let part2: u32 = copies_and_matches.iter().map(|(c, _m)| c).sum();
    
    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", start.elapsed());
}
