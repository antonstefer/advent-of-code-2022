use crate::util;

pub fn execute() {
    let mut priority_sum = 0;
    let lines = util::read_lines("./days/3/input.txt").unwrap();
    for line in lines {
        let line_content = line.unwrap();
        if line_content.len() % 2 != 0 {
            panic!("Line length is not even");
        }
        let (first_half, second_half) = line_content.split_at(line_content.len() / 2);
        let duplicate = find_duplicate_char(first_half, second_half);
        let priority = get_char_priority(duplicate);
        priority_sum += priority;
    }
    println!("Priority sum: {}", priority_sum);
}

fn find_duplicate_char(one: &str, two: &str) -> char {
    for c in one.chars() {
        if two.contains(c) {
            return c;
        }
    }
    panic!("No duplicate char found");
}

fn get_char_priority(c: char) -> usize {
    let value = c as usize;
    // if char is capital
    if value >= 65 && value <= 90 {
        // return 27 for A, 28 for B, 29 for C, ...
        return value - 38;
    }
    // if char is lowercase
    if value >= 97 && value <= 122 {
        // return 1 for a, 2 for b, 3 for c, ...
        return value - 96;
    }
    panic!("Char is not a letter");
}
