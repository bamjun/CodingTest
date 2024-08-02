use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let m: i32 = input.trim().parse().unwrap();
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let n: i32 = input2.trim().parse().unwrap();
    
    let mut prime_list: Vec<i32> = Vec::new();
    
    for i in m..=n {
        if is_prime(i) {
            prime_list.push(i);
        }
    }
    
    if !prime_list.is_empty() {
        let sum: i32 = prime_list.iter().sum();
        let min: i32 = *prime_list.iter().min().unwrap();
        println!("{}", sum);
        println!("{}", min);
    } else {
        // 소수가 없다면 -1을 출력합니다.
        println!("-1");
    }

}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}