

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn main() {
    let input = include_str!("../../../../inputs/day-03/input1.txt");
//    let input = include_str!("input.txt");

    let mut a = 0;
    let mut engine :Vec<Vec<u8>> = Vec::new();
    for l in input.split('\n') {
        if l.len() == 0 {
            continue;
        }
        engine.push(l.as_bytes().to_vec());
    }

    let mut i = 0;
    while i < engine.len() {
        let mut j = 0;
        let len = engine[i].len();
        while j < len {
            if engine[i][j].is_ascii_digit() {
                let s = j;
                j += 1;
                while j < len && engine[i][j].is_ascii_digit() {
                    j += 1;
                }
                let start = if s == 0 { 0 } else { s - 1 };
                let end = if j == len { j } else { j + 1 };
                let vstart = if i == 0 { 0 } else { i - 1 };
                let vend = if i == engine.len() - 1 { i+1 } else { i + 2 };
                for ii in vstart..vend {
                    for jj in start..end {
                        if is_symbol(engine[ii][jj]) {
                            let num = String::from_utf8(engine[i][s..j].to_vec()).unwrap();
                            a += num.parse::<u32>().unwrap();
                        }
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    

    println!("{a}")
}
