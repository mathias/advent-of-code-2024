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

fn split_line_with_regex(line: &str) -> (u32, u32) {
    let re = Regex::new(r"^(\d+)[\s]+(\d+)$").expect("Should be valid");

    let Some(caps) = re.captures(line) else { panic!("Shouldn't reach this") };

    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

fn part_1() {
    let lines = parse_file("inputs/day1.txt");
    println!("{}", distances(lines))
}

fn part_2() {
    let lines = parse_file("inputs/day1.txt");
    println!("{}", part_2_similarity(lines))
}


fn part_2_similarity(lines: Vec<String>) -> u32 {
    let mut first_list: Vec<u32> = vec![];
    let mut second_list: Vec<u32> = vec![];
    let mut outputs = vec![];

    for line in lines {
        let parts: (u32, u32) = split_line_with_regex(&line);
        first_list.push(parts.0);
        second_list.push(parts.1);

    }

    for num in first_list {
        outputs.push(single_similarity(num, second_list.clone()));
    }
    outputs.into_iter().sum()
}

fn single_similarity(num: u32, right: Vec<u32>) -> u32 {
    let mut count = 0;
    for x in right {
        if x == num {
            count += 1;
        }
    }
    count * num
}

pub fn main(part: usize) {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Ayeee")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse_lines_from_str;

    #[test]
    fn part_1_example() {
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


    #[test]
    fn part_2_example() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let lines = parse_lines_from_str(input);
        assert_eq!(part_2_similarity(lines), 31);
    }

    #[test]
    fn similarity_scores_test() {
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(single_similarity(3, right.clone()), 9);
        assert_eq!(single_similarity(4, right.clone()), 4);
        assert_eq!(single_similarity(2, right.clone()), 0);
    }
}
