use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut reader = io::stdin().lock().lines();
    let mut writer = BufWriter::new(io::stdout().lock());
    
    while let Some(Ok(line)) = reader.next() {
        let nums: Vec<i32> = line.split_whitespace().map(|I| I.parse().unwrap()).collect();
        let sum = nums[0] + nums[1];
        writeln!(writer, "{}", sum).unwrap();
    }
    writer.flush().unwrap();
}