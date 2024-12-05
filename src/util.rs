use std::fs;

pub fn parse_file(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.trim().lines() {
        lines.push(line.to_string());
    }
    lines
}

pub fn parse_lines_from_str(input: &str) -> Vec<String> {
    input.split("\n").into_iter().map(|s| s.to_string() ).collect::<Vec<String>>()
}
