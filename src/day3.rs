use regex::Regex;
use std::path::Path;
use crate::util::parse_file;

//fn path() -> Path {
    //Path::new("inputs/day3.txt")
//}
const PATH: &str = "inputs/day3.txt";

fn part_1() {
    let mut totals: u32 = 0;
    let lines = parse_file(PATH);

    for line in lines {
        totals += mult_pairs(parse_mul_pairs(&line));
    }

    println!("{}", totals)
}
//fn part_2() {}

pub fn main(part: usize) {
    match part {
        1 => part_1(),
        //2 => part_2(),
        _ => panic!("Ayeee")
    }
}

fn parse_mul_pairs(line: &str) -> Vec<(u32, u32)> {
    let mut output: Vec<(u32, u32)> = vec![];
    let outer_re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Should be valid");
    let inner_re = Regex::new(r"(\d+)\,(\d+)").expect("Should be valid");

    let matches = outer_re.find_iter(line).map(|mat| mat.as_str()).collect::<Vec<&str>>();

    for mat in matches {
        let Some(caps) = inner_re.captures(mat) else { panic!("Shouldn't reach this, in: {}", mat) };

        let pair = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        output.push(pair);
    }

    output
}

fn mult_pairs(vals: Vec<(u32, u32)>) -> u32 {
    //vals.into_iter().reduce(|acc, (x, y)| acc += x * y )
    let mut nums = vec![];

    for (x, y) in vals {
        nums.push(x * y);
    }

    nums.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    //use crate::util::parse_lines_from_str;

    #[test]
    fn test_matches_numbers() {
        let input_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected: Vec<(u32, u32)> = vec![(2,4), (5,5), (11,8), (8,5)];

        assert_eq!(expected, parse_mul_pairs(input_str));
    }

    // Only the four highlighted sections are real mul instructions.
    // Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
    #[test]
    fn test_mults_pairs() {
        let vals: Vec<(u32, u32)> = vec![(2,4), (5,5), (11,8), (8,5)];

        assert_eq!(161, mult_pairs(vals));
    }
}
