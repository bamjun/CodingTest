use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut reader = io::stdin().lock().lines();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t: usize = reader.next().unwrap().unwrap().trim().parse().expect("Failed to parse number");
    
    for i in 1..=t {
        if let Some(Ok(line)) = reader.next() {
            let nums: Vec<i32> = line.split_whitespace().map(|I| I.parse().expect("Failed to parse number")).collect();
            let a = nums[0];
            let b = nums[1];
            let sum = a+b;
            writeln!(writer, "Case #{}: {} + {} = {}", i, a, b, sum).unwrap();
        }
    }
    
    writer.flush().unwrap();
}