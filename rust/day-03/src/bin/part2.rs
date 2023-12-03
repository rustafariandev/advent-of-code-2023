

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn is_gear(c: u8) -> bool {
    c == b'*'
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
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
            if !is_gear(engine[i][j]) {
                j += 1;
                continue;
            }

            let mut nums : Vec<i32> = Vec::new();
            println!("Gear found [{i}][{j}]");
            let start = if j == 0 { 0 } else { j - 1 };
            let end = if j == len -1  { j + 1 } else { j + 2 };
            let vstart = if i == 0 { 0 } else { i - 1 };
            let vend = if i == engine.len() - 1 { i+1 } else { i + 2 };
            for ii in vstart..vend {
                let mut jj = start;
                while jj < end {
                    //print!("{}", char::from_u32(engine[ii][jj] as u32).unwrap());
                    if !is_digit(engine[ii][jj]) {
                        jj+=1;
                        continue;
                    }

                    let mut s = jj;
                    let mut e = jj;
                    while s > 0 && is_digit(engine[ii][s]) {
                        s -= 1;
                    }
                    if !engine[ii][s].is_ascii_digit() {
                        s += 1;
                    }
                    while e < engine[ii].len() && engine[ii][e].is_ascii_digit() {
                        e += 1;
                    }
                    let num = String::from_utf8(engine[ii][s..e].to_vec()).unwrap();
                    nums.push(num.parse().unwrap());
                    jj = e;
                    continue;
                }
                println!("")
            }

            j += 1;
            if nums.len() == 2 {
                a += nums[0] * nums[1];
            }
        }
        i += 1;
    }
    

    println!("{a}")
}
