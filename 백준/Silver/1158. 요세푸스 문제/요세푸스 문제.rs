use std::collections::VecDeque;
use std::io;

fn solve_problem(n: usize, k: usize) -> String {
    let mut queue: VecDeque<usize> = (1..=n).collect();
    let mut result = Vec::new();
    
    while !queue.is_empty() {
        for _ in 0..(k - 1) {
            if let Some(front) = queue.pop_front() {
                queue.push_back(front);
            }
        }
        
        if let Some(removed) = queue.pop_front() {
            result.push(removed);
        }
    }
    
    format!("<{}>", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let inputs: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let n = inputs[0];
    let k = inputs[1];
    
    let result = solve_problem(n, k);
    println!("{}", result);
    
}