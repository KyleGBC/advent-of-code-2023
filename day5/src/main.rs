#[derive(Debug, Clone)]
struct Map { dest: u32, src: u32, len: u32 }
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
    fn map_seed(&self, seed: u32) -> Option<u32>
    {
        let diff = seed.checked_sub(self.src)?;
        return if diff > self.len { None } else { Some(self.dest + diff) }
    }
}
type Layer = Vec<Map>;

fn map_seed_layer(layer: &Layer, seed: u32) -> u32
{
    for map in layer { if let Some(mapped) = map.map_seed(seed) { return mapped } }
    return seed
}

fn map_seed_almanac(mut seed: u32, layers: &Vec<Layer>) -> u32
{
    layers.iter().for_each(|l| seed = map_seed_layer(l, seed) );
    seed
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u32> = seeds.split_whitespace().map(|n| n.parse().unwrap()).collect();

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
    println!("{part1}");

}
