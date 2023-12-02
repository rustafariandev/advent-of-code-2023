use std::str::FromStr;

#[derive(Debug, Default)]
struct Count {
    red:i32,
    green:i32,
    blue:i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCountError;

impl FromStr for Count {
    type Err = ParseCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out : Count = Default::default();
        for spec in s.split(", ") {
            let (count, color) = spec.split_once(" ").ok_or(ParseCountError)?;
            let count = count.parse::<i32>().map_err(|_| ParseCountError)?;
            match color {
                "blue" => out.blue = count,
                "green" => out.green = count,
                "red" => out.red = count,
                _ => todo!(),
            }
        }
        Ok(out)
    }
}

fn main() {
    let input = include_str!("../../../../inputs/day-02/input1.txt");
    let mut a = 0;
    'game: for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let (game, rest) = l.split_once(": ").unwrap();
        println!("{game}");
        for draw in rest.split("; ") {
            if let Ok(count) = Count::from_str(draw) {
                if count.red > 12 || count.green > 13 || count.blue > 14 {
                    continue 'game;
                }
            }
        }

        let (_, num) = game.split_once(" ").unwrap();
        let num = num.parse::<i32>().unwrap();
        a += num;
    }

    println!("{a}")
}
