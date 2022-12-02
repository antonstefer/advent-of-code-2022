use crate::util;

pub fn execute() {
    let mut score = 0;
    let lines = util::read_lines("./days/2/input.txt").unwrap();
    for line in lines {
        let line_content = line.unwrap();
        // read first and third character
        let opponent_choice = line_content.chars().nth(0).unwrap();
        let game_result = line_content.chars().nth(2).unwrap();
        score += get_my_round_score(game_result, opponent_choice);
    }
    println!("Score: {}", score);
}

fn get_my_round_score(game_result: char, opponent_choice: char) -> usize {
    let opponent_index: usize = match opponent_choice {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => panic!("Opponent choice is not A, B or C"),
    };
    let game_result_index: usize = match game_result {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("Game result is not X, Y or Z"),
    };

    const SCORE_TABLE: [[usize; 3]; 3] = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

    return SCORE_TABLE[opponent_index][game_result_index] + game_result_index * 3;
}
