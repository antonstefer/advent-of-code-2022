use crate::util;

pub fn execute() {
    let mut score = 0;
    let lines = util::read_lines("./days/2/input.txt").unwrap();
    for line in lines {
        let line_content = line.unwrap();
        // read first and third character
        let opponent_choice = line_content.chars().nth(0).unwrap();
        let my_choice = line_content.chars().nth(2).unwrap();
        score += get_my_round_score(my_choice, opponent_choice);
    }
    println!("Score: {}", score);
}

fn get_my_round_score(my_choice: char, opponent_choice: char) -> usize {
    let opponent_index: usize = match opponent_choice {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => panic!("Opponent choice is not A, B or C"),
    };
    let my_index: usize = match my_choice {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("My choice is not X, Y or Z"),
    };

    const SCORE_TABLE: [[usize; 3]; 3] = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];

    return SCORE_TABLE[my_index][opponent_index] + my_index + 1;
}
