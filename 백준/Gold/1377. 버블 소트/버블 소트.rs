use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 번째 줄에서 n을 입력받습니다.
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut arr: Vec<(i32, usize)> = Vec::with_capacity(n);

    // 각 줄의 값을 읽어 배열과 인덱스를 함께 저장합니다.
    for (i, line) in lines.take(n).enumerate() {
        let value: i32 = line.unwrap().trim().parse().unwrap();
        arr.push((value, i));
    }

    // arr를 정렬합니다.
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();

    // 원래 배열과 정렬된 배열의 인덱스 차이를 구합니다.
    let mut max_diff = 0;
    for i in 0..n {
        let diff = sorted_arr[i].1 as i32 - arr[i].1 as i32;
        if diff > max_diff {
            max_diff = diff;
        }
    }

    // 결과 출력
    println!("{}", max_diff + 1);
}
