use crate::util;
use std::collections::HashSet;

pub fn execute() {
    let mut lines = util::read_lines("./days/6/input.txt").unwrap().peekable();
    let first = lines.peek().unwrap().as_ref().unwrap();

    let chars = first.chars().collect::<Vec<char>>();
    let mut windows = chars.windows(4);
    let mut index = 1;

    for _char in chars.iter() {
        if index < 4 || contains_duplicates(windows.next().unwrap()) {
            index += 1;
            continue;
        } else {
            break;
        }
    }

    println!("{}", index);
}

// check if array of 4 contains duplicates
fn contains_duplicates(array: &[char]) -> bool {
    let mut set = HashSet::new();
    for &item in array.iter() {
        if !set.insert(item) {
            return true;
        }
    }
    false
}
