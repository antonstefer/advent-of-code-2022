use crate::util;

pub fn execute() {
    let mut completely_overlapping_ranges = 0;
    let lines = util::read_lines("./days/4/input.txt").unwrap();
    for line in lines {
        let line_content = line.unwrap();
        let split: Vec<&str> = line_content.split(",").collect();
        let range_one_bounds: Vec<&str> = split[0].split("-").collect();
        let range_two_bounds: Vec<&str> = split[1].split("-").collect();
        let range_one_lower_bound = range_one_bounds[0].parse::<usize>().unwrap();
        let range_one_upper_bound = range_one_bounds[1].parse::<usize>().unwrap();
        let range_two_lower_bound = range_two_bounds[0].parse::<usize>().unwrap();
        let range_two_upper_bound = range_two_bounds[1].parse::<usize>().unwrap();
        if ranges_overlap_completely(
            (range_one_lower_bound, range_one_upper_bound),
            (range_two_lower_bound, range_two_upper_bound),
        ) {
            completely_overlapping_ranges += 1;
        }
    }
    println!("Overlapping ranges: {}", completely_overlapping_ranges);
}

fn ranges_overlap_completely(range_one: (usize, usize), range_two: (usize, usize)) -> bool {
    if range_one.0 <= range_two.0 && range_one.1 >= range_two.1 {
        return true;
    }
    if range_two.0 <= range_one.0 && range_two.1 >= range_one.1 {
        return true;
    }
    return false;
}
