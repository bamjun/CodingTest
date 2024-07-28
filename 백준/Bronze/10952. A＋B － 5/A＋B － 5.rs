use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let mut reader = io::stdin().lock().lines();
    let mut writer = BufWriter::new(io::stdout().lock());
    
    while let Some(Ok(line)) = reader.next() {
        let nums: Vec<i32> = line.split_whitespace().map(|I| I.parse().unwrap()).collect();
        let a = nums[0];
        let b = nums[1];


        if a == 0 && b == 0 {
            break;
        }
        let sum = a + b;
        writeln!(writer, "{}", sum).unwrap();

    }

    
    writer.flush().unwrap();
    
}