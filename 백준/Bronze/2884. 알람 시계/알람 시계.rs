use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    // Using i32 for safety in range and parsing
    let n: Vec<i32> = input
        .split_whitespace()
        .map(|o| o.parse().expect("failed to parse number"))
        .collect();

    let (hours, minutes) = (n[0], n[1]);
    let total_minutes = hours * 60 + minutes;
    let wakeup_time = total_minutes - 45;

    // Adjust for negative wakeup time (the day before)
    let wakeup_time = if wakeup_time < 0 {
        24 * 60 + wakeup_time
    } else {
        wakeup_time
    };

    let wake_h = (wakeup_time / 60) % 24;
    let wake_m = wakeup_time % 60;

    println!("{} {}", wake_h, wake_m);
}
