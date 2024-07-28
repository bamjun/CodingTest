use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock().lines();
    let mut writer = BufWriter::new(stdout.lock());

    while let Some(Ok(line)) = reader.next() {
        let nums: Vec<i32> = line.split_whitespace()
                                 .map(|s| s.parse().expect("Failed to parse number"))
                                 .collect();
        let a = nums[0];
        let b = nums[1];

        if a == 0 && b == 0 {
            break;
        }

        writeln!(writer, "{}", a + b).expect("Failed to write to buffer");
    }

    writer.flush().expect("Failed to flush buffer");
}
