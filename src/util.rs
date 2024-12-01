use std::fs;

pub fn parse_file(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        lines.push(line.to_string());
    }
    lines
}
