use std::str::FromStr;

#[derive(Debug)]
struct CardPlay {
    winning: Vec<u32>,
    numbers: Vec<u32>,
    copies: u32,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for CardPlay {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning, numbers) = s.split_once(" | ").ok_or(ParseError)?;
        let winning: Vec<u32> = winning.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        let numbers: Vec<u32> = numbers.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        Ok(
            CardPlay {
            winning,
            numbers,
            copies:1
            }
        )
    }
}

impl CardPlay {
    fn matches(&self) -> usize {
        self.winning.iter().fold(0, |count, i| {
            count + if self.numbers.iter().find(|&&x| x == *i).is_some() {1} else {0}
        })
    }
}

fn main() {
    let input = include_str!("../../../../inputs/day-04/input1.txt");
//    let input = include_str!("input.txt");
    let mut cards: Vec<CardPlay> = Vec::new();
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let (_name, data) = l.split_once(": ").unwrap();
        cards.push(CardPlay::from_str(data).unwrap());
    }

    for i in 0..cards.len() {
        let matches = cards[i].matches();
        for j in i+1..i+1+matches {
            cards[j].copies += cards[i].copies;
        }
    }

    println!(
        "{}",
        cards.iter().fold(0, |count, i| count + i.copies)
    );

}
