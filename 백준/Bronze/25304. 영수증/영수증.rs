use std::io;

fn main() {
    let mut total_input = String::new();
    io::stdin().read_line(&mut total_input).expect("Failed to read line");
    let total: i32 = total_input.trim().parse().expect("Failed to parse number");
    
    let mut type_input = String::new();
    io::stdin().read_line(&mut type_input).expect("Failed to read line");
    let type_number: u8 = type_input.trim().parse().expect("Failed to read line");
    
    let mut total_amount: i32 = 0;
    
    for i in 1..=type_number {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let price_list: Vec<i32> = input.split_whitespace().map(|O| O.parse().expect("Failed to parse number")).collect();
        
        let price: i32 = price_list[0];
        let count: i32 = price_list[1];
        
        total_amount += price * count;
    }
    
    if total == total_amount {
        println!("Yes");
    } else {
        println!("No");
    };
    
}