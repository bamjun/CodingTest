use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    // Read the first word
    io::stdin().read_line(&mut input).unwrap();
    let word1 = input.trim().to_string();

    input.clear();

    // Read the second word
    io::stdin().read_line(&mut input).unwrap();
    let word2 = input.trim().to_string();

    // Calculate character frequencies
    let freq1 = char_frequency(&word1);
    let freq2 = char_frequency(&word2);

    // Calculate the minimum number of characters to remove
    let result = calculate_min_removals(&freq1, &freq2);
    println!("{}", result);
}

// Calculate character frequency
fn char_frequency(word: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in word.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

// Calculate minimum removals
fn calculate_min_removals(freq1: &HashMap<char, usize>, freq2: &HashMap<char, usize>) -> usize {
    let mut removals = 0;

    // Collect all unique characters from both frequency maps
    let mut all_chars: Vec<char> = freq1.keys().chain(freq2.keys()).copied().collect();
    all_chars.sort();
    all_chars.dedup();

    // Compare frequencies and calculate removals
    for ch in all_chars {
        let count1 = freq1.get(&ch).copied().unwrap_or(0);
        let count2 = freq2.get(&ch).copied().unwrap_or(0);
        removals += (count1 as isize - count2 as isize).abs() as usize;
    }

    removals
}
