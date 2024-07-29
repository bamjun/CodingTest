use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock().lines();
    let mut writer = BufWriter::new(stdout.lock());

    while let Some(Ok(line)) = reader.next() {
        let n: i32 = line.trim().parse().unwrap();
        if n == -1 {
            break;
        }
        
        let mut divisors: Vec<i32> = Vec::new();
        
        for i in 1..n { 
            if n % i == 0 {
                divisors.push(i);
            }
        }
        
        let sum: i32 = divisors.iter().sum();
        
        if sum == n {
            let divisors_str: String = divisors.iter()
                                               .map(|d| d.to_string())
                                               .collect::<Vec<String>>()
                                               .join(" + ");
            writeln!(writer, "{} = {}", n, divisors_str).unwrap();
        } else {
            writeln!(writer, "{} is NOT perfect.", n).unwrap();
        }
    }
}
