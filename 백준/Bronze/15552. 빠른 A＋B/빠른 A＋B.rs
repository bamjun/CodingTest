use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let t: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        if let Some(Ok(line)) = reader.next() {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let sum = nums[0] + nums[1];
            writeln!(writer, "{}", sum).unwrap();
        }
    }

    writer.flush().unwrap();
}
