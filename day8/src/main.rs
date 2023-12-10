use std::collections::HashMap;

fn main() {
    let start = std::time::Instant::now();
    let mut input = include_str!("../input.txt").lines();
    let mut sequence = input.next().unwrap().chars().cycle();

    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::with_capacity(750);
    for line in input.skip(1)
    {
        let src = &line[0..3];
        let left = &line[7..10];
        let right= &line[12..15];
        mappings.insert(src, (left, right));
    }

    let mut position = "AAA";
    let mut steps = 0;
    while position != "ZZZ"
    {
        let (left, right) = mappings.get(position).expect("Couldn't find mapping from a position}");
        let dir = sequence.next().unwrap();
        position = if dir == 'L' { *left } else { *right };
        steps += 1;
    }
    println!("{steps}, in {:?}", start.elapsed());
}
