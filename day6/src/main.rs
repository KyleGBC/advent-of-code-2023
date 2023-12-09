fn find_wait_bounds(total_time: f64, record_distance: f64) -> (u32, u32)
{
    let determinant = ((total_time.powf(2.0) - 4.0 * record_distance)).sqrt();
    let upper = ((-1.0 * total_time - determinant) / -2.0) - 0.000001;
    let lower = ((-1.0 * total_time + determinant) / -2.0) + 0.000001;
    return (lower.ceil() as u32, upper.floor() as u32)
}

fn main() {
    let start = std::time::Instant::now();
    let mut input = include_str!("../input.txt").lines();
    let times = input.next().unwrap().split_whitespace().skip(1);
    let distances = input.next().unwrap().split_whitespace().skip(1);

    let part1: u32 = times.clone().map(|n| n.parse::<f64>().unwrap()).zip(distances.clone().map(|n| n.parse::<f64>().unwrap())).map(|(t, d)| find_wait_bounds(t, d)).map(|(l, u)| 1 + u - l).product();
    let part2 = find_wait_bounds(times.collect::<String>().parse::<f64>().unwrap(), distances.collect::<String>().parse::<f64>().unwrap());
    let part2 = 1 + part2.1 - part2.0;

    println!("{part1}, {part2} in {:?}", start.elapsed());
}
