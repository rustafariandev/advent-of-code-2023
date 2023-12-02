use std::str::FromStr;

#[derive(Debug, Default)]
struct Count {
    red:i64,
    green:i64,
    blue:i64,
}


impl Count {
    fn power(&self) -> i64 {
        return self.red * self.green * self.blue
    }

    fn merge(&mut self, o:&Self) {
        if self.red < o.red {
            self.red = o.red
        }
        if self.green < o.green {
            self.green = o.green
        }
        if self.blue < o.blue {
            self.blue = o.blue
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCountError;

impl FromStr for Count {
    type Err = ParseCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out : Count = Default::default();
        for spec in s.split(", ") {
            let (count, color) = spec.split_once(" ").ok_or(ParseCountError)?;
            let count = count.parse::<i64>().map_err(|_| ParseCountError)?;
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
    let mut a:i64 = 0;
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let (game, rest) = l.split_once(": ").unwrap();
        println!("{game}");
        let mut merge : Count = Default::default();
        for draw in rest.split("; ") {
            if let Ok(count) = Count::from_str(draw) {
                merge.merge(&count)
            }
        }

        a += merge.power();
    }

    println!("{a}")
}
