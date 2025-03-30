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

    // Calculate the diff between the two number
    left.sort();
    right.sort();

    let mut diff = Vec::new();
    for i in 0..left.len() {
        // Avoid negative result
        diff.push((left[i] - right[i]).abs());
    }
    println!("{:?}", diff);

    let diff_sum: i64 = diff.iter().sum();
    println!("Total diff: {}", diff_sum);
}
