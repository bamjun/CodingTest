use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 번째 줄: 테스트 케이스 수
    let number_of_test_case: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected a valid integer");

    // 결과를 저장할 벡터
    let mut results = Vec::new();

    for _ in 0..number_of_test_case {
        if let Some(Ok(line)) = lines.next() {
            let words: Vec<&str> = line.trim().split_whitespace().collect();
            if words.len() == 2 {
                let mut first = words[0].chars().collect::<Vec<char>>();
                let mut second = words[1].chars().collect::<Vec<char>>();
                first.sort_unstable();
                second.sort_unstable();
                if first == second {
                    results.push("Possible");
                } else {
                    results.push("Impossible");
                }
            }
        }
    }

    // 결과 출력
    for result in results {
        println!("{}", result);
    }
}
