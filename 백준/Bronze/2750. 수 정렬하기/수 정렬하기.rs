use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed readline");
    let n: usize = input.trim().parse().expect("not number");
    
    let mut nums = Vec::new();
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed readline");
        let num: i32 = input.trim().parse().expect("not number");
        nums.push(num)
    }
    
    nums.sort();
    
    for num in nums {
        println!{"{}", num};
    }
}