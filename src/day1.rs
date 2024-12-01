use regex::Regex;
use crate::util::parse_file;

fn distances(inputs: Vec<String>) -> u32 {
    let mut first_list: Vec<u32> = vec![];
    let mut second_list: Vec<u32> = vec![];
    let mut outputs = vec![];

    for line in inputs {
        let parts: (u32, u32) = split_line_with_regex(&line);
        first_list.push(parts.0);
        second_list.push(parts.1);

    }
    first_list.sort();
    second_list.sort();

    for (idx, fi) in first_list.iter().enumerate() {
        let sec = second_list[idx];
        if *fi > sec {
            outputs.push(fi - sec);
        } else {
            outputs.push(sec - fi);
        }
    }

    outputs.into_iter().sum()
}

fn parse_lines_from_str(input: &str) -> Vec<String> {
    input.split("\n").into_iter().map(|s| s.to_string() ).collect::<Vec<String>>()
}


fn split_line_with_regex(line: &str) -> (u32, u32) {
    let re = Regex::new(r"^(\d+)[\s]+(\d+)$").expect("Should be valid");

    let Some(caps) = re.captures(line) else { panic!("Shouldn't reach this") };

    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

pub fn main() {
    let lines = parse_file("inputs/day1.txt");
    println!("{}", distances(lines))
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example_input() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let lines = parse_lines_from_str(input);

        assert_eq!(distances(lines), 11);
    }
}
