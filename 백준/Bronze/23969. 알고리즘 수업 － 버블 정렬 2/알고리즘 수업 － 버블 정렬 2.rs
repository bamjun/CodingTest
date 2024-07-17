use std::io::{self, BufRead};

fn bubble_sort_kth_swap(n: usize, k: usize, mut a: Vec<i32>) -> Option<Vec<i32>> {
    let mut swap_count = 0;
    
    for last in (1..n).rev() {
        for i in 0..last {
            if a[i] > a[i + 1] {
                a.swap(i, i + 1);
                swap_count += 1;
                if swap_count == k {
                    return Some(a);
                }
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let first_line = iterator.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    
    let second_line = iterator.next().unwrap().unwrap();
    let a: Vec<i32> = second_line.split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect();
    
    match bubble_sort_kth_swap(n, k, a) {
        Some(result) => println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),
        None => println!("-1"),
    }
}
