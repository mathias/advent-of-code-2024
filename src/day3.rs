use regex::Regex;
use crate::util::parse_file;

const PATH: &str = "inputs/day3.txt";

fn part_1() {
    let mut totals: u32 = 0;
    let lines = parse_file(PATH);

    for line in lines {
        totals += mult_pairs(parse_mul_pairs(&line));
    }

    println!("{}", totals)
}

fn part_2() {
    let input_with_newlines = parse_file(PATH);
    let input_cleaned = part_2_clean_and_join(input_with_newlines);

    println!("{}", part_2_glue(&input_cleaned))
}

fn part_2_clean_and_join(input: Vec<String>) -> String {
    let input_joined = input.join("");
    let re = Regex::new("\n").expect("Should parse");
    re.split(&input_joined).collect::<Vec<&str>>().join("")
}

pub fn main(part: usize) {
    match part {
        1 => part_1(),
        2 => part_2(),
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

fn filter_do_donts(input: &str) -> String {
    let re = Regex::new(r"(don't\(\).*?do\(\))").expect("Should be valid");


    // filter out when don't() goes to end of line
    let second_re = Regex::new(r"(don't\(\).*?)$").expect("Should be valid");
    let partial = re.replace_all(input, "").into_owned();
    second_re.replace_all(&partial, "").into_owned()
}

fn part_2_glue(input: &str) -> u32 {
    //let mut pairs: Vec<(u32, u32)> = vec![];

    //for line in filter_do_donts(input) {
        //pairs.append(&mut parse_mul_pairs(line));
    //}
    let pairs = parse_mul_pairs(&filter_do_donts(input));

    mult_pairs(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::util::parse_lines_from_str;

    #[test]
    fn parse_mul_pairs_test() {
        let input_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected: Vec<(u32, u32)> = vec![(2,4), (5,5), (11,8), (8,5)];

        assert_eq!(expected, parse_mul_pairs(input_str));
    }

    // Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
    #[test]
    fn mults_pairs_test() {
        let vals: Vec<(u32, u32)> = vec![(2,4), (5,5), (11,8), (8,5)];

        assert_eq!(161, mult_pairs(vals));
    }

    #[test]
    fn part_2_example() {
        //There are two new instructions you'll need to handle:

        //The do() instruction enables future mul instructions.
        //The don't() instruction disables future mul instructions.

        //Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

        //For example:

        //xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

        //This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

        //This time, the sum of the results is 48 (2*4 + 8*5).
        let input_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 48;

        assert_eq!(expected, part_2_glue(input_str));
    }

    #[test]
    fn part_2_match_do_donts() {
        let input_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = "xmul(2,4)&mul[3,7]!^?mul(8,5))";
        assert_eq!(expected, filter_do_donts(input_str));
    }

    #[test]
    fn part_2_match_do_donts_not_greedy() {
        let input_str = "xmul(1,2)_don't()_mul(3,4)_do()mul(5,6).do()?mul(7,8)";
        let expected = "xmul(1,2)_mul(5,6).do()?mul(7,8)";
        assert_eq!(expected, filter_do_donts(input_str));
    }

    #[test]
    fn part_2_match_do_donts_ending() {
        let input_str = "xmul(1,2)_don't()_mul(3,4)_mul(5,6).mul(7,8)";
        let expected = "xmul(1,2)_";
        assert_eq!(expected, filter_do_donts(input_str));
    }

    #[test]
    fn part_2_match_do_donts_repeats() {
        let input_str = "xmul(1,2)_don't()_mul(3,4)_do()*inbetween*don't()mul(5,6).mul(7,8)";
        let expected = "xmul(1,2)_*inbetween*";
        assert_eq!(expected, filter_do_donts(input_str));
    }

    #[test]
    fn part_2_clean_and_join_test() {
        let input = "\
foo
_
bar";
        let parsed_input = parse_lines_from_str(input);
        let expected = "foo_bar";
        assert_eq!(expected, part_2_clean_and_join(parsed_input));
    }

    #[test]
    fn part_2_e2e_test() {
        let input = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()
mul(1,2)mul(3,4)
bar";
        let parsed_input = parse_lines_from_str(input);
        let expected = 48;

        let input_cleaned = part_2_clean_and_join(parsed_input);
        dbg!(input_cleaned.clone());

        assert_eq!(expected, part_2_glue(&input_cleaned));
    }
}
