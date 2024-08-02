use std::io;

fn main() {
    // 입력값을 받을 두 개의 변수를 정의합니다.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let m: i32 = input.trim().parse().unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let n: i32 = input2.trim().parse().unwrap();

    // 소수를 저장할 리스트를 정의합니다.
    let mut prime_list: Vec<i32> = Vec::new();

    // M부터 N까지의 숫자에 대해 소수 여부를 확인합니다.
    for i in m..=n {
        if is_prime(i) {
            prime_list.push(i);
        }
    }

    // 소수 리스트가 비어있지 않다면 합과 최솟값을 출력합니다.
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

// 소수 판별 함수입니다.
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false; // 1 이하의 숫자는 소수가 아님
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false; // 나누어 떨어지면 소수가 아님
        }
    }
    true // 나누어 떨어지지 않으면 소수임
}
