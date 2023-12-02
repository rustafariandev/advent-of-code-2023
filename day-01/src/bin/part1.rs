fn main() {
    let input = include_str!("input1.txt");
    let mut a = 0;
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let mut n = String::new();
        if let Some(c) = l.chars().find(|c| c.is_digit(10)) {
            n.push(c);
        }

        if let Some(c) = l.chars().rfind(|c| c.is_digit(10)) {
            n.push(c);
        }

        if let Ok(i) = n.parse::<i32>() {
            a += i
        }
    }
    println!("{a}")
}
