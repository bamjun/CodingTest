use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: Vec<i32> = input.split_whitespace().map(|i| i.parse().expect("failed to parse number")).collect();
    let (h, m) = (n[0], n[1]);
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read line");
    let need_time: i32 = input2.trim().parse().expect("failed to parse number");
    
    let total: i32 = h * 60 + m + need_time;
    
    let final_h = ( total / 60 ) % 24;
    let final_m = total % 60;
    println!("{} {}", final_h, final_m);

}