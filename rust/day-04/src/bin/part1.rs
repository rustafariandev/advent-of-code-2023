


fn main() {
    let input = include_str!("../../../../inputs/day-04/input1.txt");
//    let input = include_str!("input.txt");
    let mut a = 0;
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }
        let (_name, data) = l.split_once(": ").unwrap();
        let (winning, numbers) = data.split_once(" | ").unwrap();
        let winning: Vec<u32> = winning.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        let numbers: Vec<u32> = numbers.split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        let found = winning.iter().fold(0, |count, i| {
            count + if numbers.iter().find(|&&x| x == *i).is_some() {1} else {0}
        });

        if found > 0 {
            a += 1 <<  (found - 1);
        }
    }
    println!("{a}")
}
