use rayon::prelude::*;

struct SeedRange { start: u64, end: u64 }


#[derive(Debug, Clone)]
struct Map { dest: u64, src: u64, len: u64 }
impl Map
{
    fn parse(s: &str) -> Map
    {
       let mut iter = s.split_whitespace();
       let dest = iter.next().unwrap().parse().unwrap();
       let src = iter.next().unwrap().parse().unwrap();
       let len = iter.next().unwrap().parse().unwrap();
       Map { dest, src, len}
    }
    fn map_seed(&self, seed: u64) -> Option<u64>
    { 
        let diff = seed.checked_sub(self.src)?;
        return if diff > self.len { None } else { Some(self.dest + diff) }
    }
    fn contains(self, seed: u64) -> bool { return self.src <= seed && seed <= self.src + self.len }
}
type Layer = Vec<Map>;

fn map_seed_layer(layer: &Layer, seed: u64) -> u64
{
    for map in layer { if let Some(mapped) = map.map_seed(seed) { return mapped } }
    return seed
}

fn map_seed_almanac(mut seed: u64, layers: &Vec<Layer>) -> u64
{
    layers.iter().for_each(|l| seed = map_seed_layer(l, seed) );
    seed
}

fn process_range_layer(mut range: SeedRange, layer: &Layer) -> Vec<SeedRange>
{
    let mut ranges = Vec::new();
    let i = 0;
    while i < ranges.len()
    {
        let current_range = ranges[i];

        for map in layer
        {
           // Five cases:
           // Case 1: Wholly contained withing mapping range
           // Case 2 Left contained, right not contained
           // Case 3: Right contained, left not contained
           // Case 4: Mapping contained wholly withing seed range, necessitates three-way split
           // Case 5: No overlap
        }
    }
    ranges
}

fn process_range(mut range: SeedRange, layers: &Vec<Layer>) -> Vec<SeedRange>
{
    let mut ranges = Vec::new();
    for layer in layers { ranges = process_range_layer(range, layer); }
    ranges
}

fn main() {
    let time = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u64> = seeds.split_whitespace().map(|n| n.parse().unwrap()).collect();

    lines.next();

    let mut layers: Vec<Layer> = Vec::new();
    let mut layer = Layer::with_capacity(40);
    for line in lines
    {
        if line.is_empty() { continue;}

        let starts_with = line.chars().next().unwrap();

        if starts_with.is_digit(10)
        {
            layer.push(Map::parse(line));
        }
        else
        {
            if !layer.is_empty() { layers.push(layer.clone()); layer.clear(); }
        }
    }
    layers.push(layer);
    
    let part1 = seeds.iter().map(|s| map_seed_almanac(*s, &layers)).min().unwrap();

    let mut seed_ranges: Vec<SeedRange> = seeds.chunks(2).map(|v| SeedRange { start: v[0], end: v[1] }).map(|s| process_range(s, &layers)).flatten().collect();


    
    println!("{part1}, {part2} in {:?}", time.elapsed());

}
