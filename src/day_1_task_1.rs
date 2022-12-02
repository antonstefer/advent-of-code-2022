use crate::util;

pub fn execute() {
    let mut highest_sum: i32 = 0;
    let mut current_sum: i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = util::read_lines("./days/1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_content) = line {
                // if line is empty reset current sum
                if line_content.is_empty() {
                    if current_sum > highest_sum {
                        highest_sum = current_sum;
                    }
                    current_sum = 0;
                } else {
                    let value: i32 = line_content.parse().unwrap();
                    current_sum += value;
                }
            }
        }
    }
    println!("Highest sum: {}", highest_sum);
}