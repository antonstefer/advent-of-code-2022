use crate::util;

pub fn execute() {
    let lines = util::read_lines("./days/5/input.txt").unwrap();
    let line_groups = util::split_at_empty_lines(lines);

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_lines = line_groups[0].clone();
    stack_lines.pop();

    fill_stacks(stack_lines, &mut stacks);

    let instruction_lines = line_groups[1].clone();

    for line in instruction_lines.iter() {
        let fragments: Vec<&str> = line.split(" ").collect();
        let amount = fragments[1].parse::<usize>().unwrap();
        let origin = fragments[3].parse::<usize>().unwrap() - 1;
        let destination = fragments[5].parse::<usize>().unwrap() - 1;

        for _i in 0..amount {
            let c = stacks[origin].pop().unwrap();
            stacks[destination].push(c);
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
}

pub fn fill_stacks(stack_lines: Vec<String>, stacks: &mut Vec<Vec<char>>) {
    for line in stack_lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        for i in (1..len).step_by(4) {
            let stack_index = (i - 1) / 4;
            if stacks.len() <= stack_index {
                stacks.push(Vec::new());
            }
            let current_char = chars[i];
            if !current_char.is_whitespace() {
                stacks[stack_index].push(current_char);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }
}
