use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut reader = io::stdin().lock().lines();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();
    
    for i in 1..=t {
        if let Some(Ok(line)) = reader.next() {
            let nums: Vec<i32> = line.split_whitespace().map(|I| I.parse().unwrap()).collect();
            let sum = nums[0] + nums[1];
            writeln!(writer, "Case #{}: {}", i, sum);
        }
    }
    
    writer.flush().unwrap();
}