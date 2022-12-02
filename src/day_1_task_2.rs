use crate::util;

pub fn execute() {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    let lines = util::read_lines("./days/1/input.txt").unwrap();
    for line in lines {
        let line_content = line.unwrap();
        if line_content.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            let value: i32 = line_content.parse().unwrap();
            current_sum += value;
        }
    }
    if current_sum != 0 {
        sums.push(current_sum);
    }
    if sums.len() < 3 {
        panic!("There are less than three sums");
    }
    sums.sort();
    sums.reverse();
    let three_highest_sums = sums[0] + sums[1] + sums[2];
    println!("Three highest sums: {}", three_highest_sums);
}
