use std::hash::BuildHasherDefault;
use fxhash::{FxHasher, FxHashMap};

fn card_value(c: char, use_jokers: bool) -> u32
{
    match c { 'A' => 14, 'K' => 13, 'Q' => 12, 'J' => if use_jokers { 1 } else { 11 } , 'T' => 10, n if n.is_digit(10) => n.to_digit(10).unwrap(), _ => unimplemented!()}
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandKind { High, OnePair, TwoPair, ThreeKind, FullHouse, FourKind, FiveKind }
impl HandKind { 

    fn from_cards(cards: &Vec<u32>, use_wilds: bool) -> Self
    {
        let mut counts: FxHashMap<u32, u32> = FxHashMap::with_capacity_and_hasher(cards.len(), BuildHasherDefault::<FxHasher>::default());

        let mut wild_count = 0;
        for card_val in cards
        {
            if use_wilds && *card_val == 1 { wild_count += 1; }
            else { counts.entry(*card_val).and_modify(|n| *n += 1).or_insert(1); }
        }
        
        let counts = counts.into_values().collect::<Vec<_>>();
        let max = counts.iter().max().unwrap_or(&0) + wild_count;
        match counts.len()
        {
            0 | 1 => Self::FiveKind,
            2 if max == 4 => Self::FourKind,
            2 => Self::FullHouse,
            3 if max == 3 => Self::ThreeKind,
            3 => Self::TwoPair,
            4 => Self::OnePair,
            5 => Self::High,
            _ => unimplemented!()
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand{ cards: Vec<u32>, bid: u32, kind: HandKind }
impl Hand
{   
    fn parse(cards: &str, bid: &str, use_wilds: bool) -> Self
    {
        let cards = cards.chars().map(|c| card_value(c, use_wilds)).collect();
        let bid = bid.parse::<u32>().unwrap();
        let kind = HandKind::from_cards(&cards, use_wilds);
        Hand { cards, bid, kind }
    }
}
impl Ord for Hand
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind != other.kind { self.kind.cmp(&other.kind) }
        else { self.cards.cmp(&other.cards) }
    }
}
impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");

    let (mut p1, mut p2): (Vec<Hand>, Vec<Hand>) = input.lines().map(|l| l.split_once(' ').unwrap()).map(|(c, b)| (Hand::parse(c, b, false), Hand::parse(c, b, true))).unzip();
    
    p1.sort();
    p2.sort();

    let part1 = p1.iter().enumerate().fold(0, |acc, (rank, hand)| acc + (rank as u32 + 1) * hand.bid);
    let part2 = p2.iter().enumerate().fold(0, |acc, (rank, hand)| acc + (rank as u32 + 1) * hand.bid);

    println!("{part1}, {part2} in {:?}", start.elapsed());

}
