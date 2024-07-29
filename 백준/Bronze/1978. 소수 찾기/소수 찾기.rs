use std::io::{self, BufRead};

fn main() {
    // 표준 입력을 읽습니다.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    
    // 첫 줄에 주어진 수의 개수 N을 읽습니다.
    let n: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();
    
    // 두 번째 줄에 주어진 N개의 수를 벡터에 저장합니다.
    let numbers: Vec<i32> = reader.next().unwrap().unwrap()
        .trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // 소수의 개수를 세기 위한 변수
    let mut prime_count = 0;
    
    // 각 숫자가 소수인지 확인합니다.
    for &num in &numbers {
        if is_prime(num) {
            prime_count += 1;
        }
    }
    
    // 결과를 출력합니다.
    println!("{}", prime_count);
}

// 소수 판별 함수
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
