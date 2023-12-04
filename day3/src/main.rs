use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Tile { Blank, Symbol(char), Number(char) }
impl Tile
{
    fn parse(c: char) -> Tile
    {
        if c == '.' { Tile::Blank }
        else if c.is_digit(10) { Tile::Number(c) }
        else { Tile::Symbol(c) }
    }
}

fn get_tile(grid: &Vec<Vec<Tile>>, x: i32, y: i32) -> Option<&Tile>
{
    let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
    grid.get(y).map(|r| r.get(x)).flatten()
}

fn neighbor_nums(grid: &Vec<Vec<Tile>>, x: i32, y: i32) -> Vec<i32>
{
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut neighbor_nums: Vec<i32> = Vec::new();

    // Iterate over neighbors
    for nx in -1..=1
    {
        for ny in -1..=1
        {
            if nx == 0 && ny == 0 || visited.contains(&(nx, ny)) { continue; }
            
            // Check if there's a number here
            if let Some(Tile::Number(c)) = get_tile(grid, x + nx, y + ny) 
            {
                let mut part_number = c.to_string();
                let mut dx = -1;

                // Prepend anything to the left
                while let Some(Tile::Number(c)) = get_tile(grid, x + nx + dx, y + ny)
                {   
                    part_number.insert(0, *c);
                    visited.insert((nx + dx, ny));
                    dx -= 1;
                   
                }
                // Append anything to the right
                dx = 1;
                while let Some(Tile::Number(c)) = get_tile(grid, x + nx + dx, y + ny)
                {
                    part_number.push(*c);
                    visited.insert((nx + dx, ny));
                    dx += 1;
                }
                neighbor_nums.push(part_number.parse::<i32>().expect("Failed to parse assembled part number"));
            }
        }
    }
    neighbor_nums
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<_>> = input.lines()
                                 .map(|l| 
                                       l.chars()
                                        .map(|c| Tile::parse(c))
                                        .collect())
                                 .collect();
    let mut part1 = 0;
    let mut part2 = 0;

    for (y, row) in grid.iter().enumerate()
    {
        for (x, tile) in row.iter().enumerate()
        {
            if let Tile::Symbol(s) = *tile
            {
                let neighbors = neighbor_nums(&grid, x as i32, y as i32);
                if s == '*' && neighbors.len() == 2
                {
                    part2 += neighbors[0] * neighbors[1];
                }
                part1 += neighbors.into_iter().sum::<i32>();
            }
        } 
    }

    println!("Part 1: {part1}.  Part 2: {part2}, in {:?}", start.elapsed());
}
