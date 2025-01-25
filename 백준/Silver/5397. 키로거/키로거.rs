use std::io;

fn process_keylogger(input: &str) -> String {
    let mut left_stack: Vec<char> = Vec::new();
    let mut right_stack: Vec<char> = Vec::new();
    
    for ch in input.chars() {
         match ch {
             '-' =>  {
                 if !left_stack.is_empty() {
                     left_stack.pop();
                 }
             }
             '<' => {
                 if !left_stack.is_empty() {
                     right_stack.push(left_stack.pop().unwrap());
                 }
             }
             '>' => {
                 if !right_stack.is_empty() {
                     left_stack.push(right_stack.pop().unwrap());
                 }
             }
             _ => {
                 left_stack.push(ch);
             }
        }
    }
    
    left_stack.extend(right_stack.into_iter().rev());
    left_stack.into_iter().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    let mut results = Vec::new();
    for _ in 0..t {
        let mut case = String::new();
        io::stdin()
            .read_line(&mut case)
            .unwrap();
        results
            .push(process_keylogger(case.trim()));
    }
    
    for result in results {
        println!("{}", result);
    }
}