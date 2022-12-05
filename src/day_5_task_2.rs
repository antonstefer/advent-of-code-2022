use crate::{day_5_task_1, util};

pub fn execute() {
    let lines = util::read_lines("./days/5/input.txt").unwrap();
    let line_groups = util::split_at_empty_lines(lines);

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_lines = line_groups[0].clone();
    stack_lines.pop();

    day_5_task_1::fill_stacks(stack_lines, &mut stacks);

    let instruction_lines = line_groups[1].clone();

    for line in instruction_lines.iter() {
        let fragments: Vec<&str> = line.split(" ").collect();
        let amount = fragments[1].parse::<usize>().unwrap();
        let origin = fragments[3].parse::<usize>().unwrap() - 1;
        let destination = fragments[5].parse::<usize>().unwrap() - 1;

        let move_up_to = stacks[origin].len() - amount;
        let mut moved = stacks[origin].split_off(move_up_to);
        stacks[destination].append(&mut moved);
    }

    for stack in stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
}
