use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read the first line containing N and K
    stdin.read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the second line containing the array elements
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let mut a: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Perform bubble sort and track the number of swaps
    let mut swap_count = 0;
    let mut swapped = true;

    for last in (1..n).rev() {
        if !swapped {
            break;
        }
        swapped = false;
        for i in 0..last {
            if a[i] > a[i + 1] {
                a.swap(i, i + 1);
                swap_count += 1;
                swapped = true;
                if swap_count == k {
                    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
                    return;
                }
            }
        }
    }

    // If the number of swaps is less than K, print -1
    println!("-1");
}
