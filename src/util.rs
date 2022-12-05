use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn split_at_empty_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    for line in lines {
        let line_content = line.unwrap();
        if line_content.is_empty() {
            groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(line_content);
        }
    }
    groups.push(current_group);
    return groups;
}
