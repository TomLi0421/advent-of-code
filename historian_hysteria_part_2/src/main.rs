use std::fs;

fn main() {
    // Store the left and right number
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Read the input
    let contents =
        fs::read_to_string("puzzle_input").expect("Should have been able to read the file");

    for line in contents.lines() {
        // Split the string on whitespace
        let numbers: Vec<&str> = line.split_whitespace().collect();

        left.push(numbers[0].parse::<i64>().unwrap());
        right.push(numbers[1].parse::<i64>().unwrap());
    }

    let mut similarity_score = 0;
    for i in 0..left.len() {
        let mut score = 0;

        for j in 0..right.len() {
            if left[i] == right[j] {
                score += left[i];
            }
        }
        similarity_score += score;
    }

    println!("Similarity score {}", similarity_score);
}
