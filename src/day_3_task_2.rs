use crate::util;

pub fn execute() {
    let mut priority_sum = 0;
    let lines = util::read_lines("./days/3/input.txt").unwrap();
    // loop over triplets of lines
    let mut line_iter = lines.peekable();
    while line_iter.peek().is_some() {
        let first_line = line_iter.next().unwrap().unwrap();
        let second_line = line_iter.next().unwrap().unwrap();
        let third_line = line_iter.next().unwrap().unwrap();
        let common_char = find_common_char(&first_line, &second_line, &third_line);
        let priority = get_char_priority(common_char);
        priority_sum += priority;
    }
    println!("Priority sum: {}", priority_sum);
}

fn find_common_char(one: &str, two: &str, three: &str) -> char {
    for c in one.chars() {
        if two.contains(c) && three.contains(c) {
            return c;
        }
    }
    panic!("No common char found");
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
