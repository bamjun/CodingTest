use std::io::{self, BufRead};

fn compute_prefix_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0; n];
    let s = s.as_bytes();
    let mut k = 0;

    for i in 1..n {
        while k > 0 && s[k] != s[i] {
            k = pi[k - 1];
        }
        if s[k] == s[i] {
            k += 1;
        }
        pi[i] = k;
    }

    pi
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    while let Some(Ok(line)) = iterator.next() {
        if line == "." {
            break;
        }

        let s = line.trim();
        let n = s.len();
        let pi = compute_prefix_function(s);

        let len = n - pi[n - 1];
        if n % len == 0 {
            println!("{}", n / len);
        } else {
            println!("1");
        }
    }
}
