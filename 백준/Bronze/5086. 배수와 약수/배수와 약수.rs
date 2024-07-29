use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut reader = io::stdin().lock().lines();
    let mut writer = BufWriter::new(io::stdout().lock());
    
    while let Some(Ok(line)) = reader.next() {
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let a = nums[0];
        let b = nums[1];

        if a == 0 && b == 0 {
            break;
        }

        if b % a == 0 {
            writeln!(writer, "factor").unwrap();
        } else if a % b == 0 {
            writeln!(writer, "multiple").unwrap();
        } else {
            writeln!(writer, "neither").unwrap();
        }
    }
}
