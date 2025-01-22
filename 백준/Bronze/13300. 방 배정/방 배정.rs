use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|X| X.parse().unwrap())
        .collect();
    
    // 학생수, 한 방의 최대 인원 수
    let (n, k) = (parts[0], parts[1]);
    
    // 학생 수를 저장할 배열 (2차원 배열: [성별][학년])    
    let mut students = vec![vec![0; 7]; 2];
    
    // 학생 데이터 입력 받기
    for _ in 0..n {
        let mut student_input = String::new();
        io::stdin().read_line(&mut student_input).unwrap();
        let student_parts: Vec<usize> = student_input
            .trim()
            .split_whitespace()
            .map(|X| X.parse().unwrap())
            .collect();
        let (s, y) = (student_parts[0], student_parts[1]);
        students[s][y] += 1;
    }
    
    let mut rooms = 0;
    for s in 0..2 {
        for y in 1..7 {
            rooms += (students[s][y] + k - 1) / k;
        }
    }
    
    println!("{}", rooms);

}