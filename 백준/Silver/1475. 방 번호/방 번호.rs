use std::io;

fn main() {
    // 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // 숫자별 개수를 저장할 배열
    let mut count = [0; 10];

    // 숫자 개수 세기
    for c in input.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        count[digit] += 1;
    }

    // 6과 9는 함께 사용할 수 있으므로 합쳐서 처리
    let six_nine_count = count[6] + count[9];
    count[6] = (six_nine_count + 1) / 2;
    count[9] = 0; // 9는 더 이상 필요 없음

    // 최댓값 계산
    let max_set = *count.iter().max().unwrap();

    println!("{}", max_set);
}
