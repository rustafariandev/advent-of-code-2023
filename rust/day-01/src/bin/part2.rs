fn main() {
    let input = include_str!("../../../../inputs/day-01/input1.txt");
    let mut a = 0;
    let lookup = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let mut i = 0;

        if let Some(a) = lookup
            .iter()
            .filter_map(|s| match l.find(s.0) {
                None => return None,
                Some(i) => Some((i, s.1)),
            })
            .min_by(|a, b| a.0.cmp(&b.0))
        {
            i = 10 * a.1;
        }

        if let Some(a) = lookup
            .iter()
            .filter_map(|s| match l.rfind(s.0) {
                None => return None,
                Some(i) => Some((i, s.1)),
            })
            .max_by(|a, b| a.0.cmp(&b.0))
        {
            i += a.1;
        }

        a += i
    }

    println!("{a}")
}
