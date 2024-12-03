use itertools::Itertools;
use crate::util::parse_file;

fn part_1() {
    let lines = parse_file("inputs/day2.txt");
    println!("{}", part_1_safe_count(lines))
}

fn part_2() {
    let lines = parse_file("inputs/day2.txt");
    println!("{}", part_2_safe_count(lines))
}


fn part_1_safe_count(lines: Vec<String>) -> usize {
    let mut count = 0;

    for line in lines {
        let nums = parse_nums(&line);
        if safe_values(nums) {
            count += 1;
        }
    }
    return count
}

fn part_2_safe_count(lines: Vec<String>) -> usize {
    let mut count = 0;

    for line in lines {
        let nums = parse_nums(&line);
        if dampener_safe(nums) {
            count += 1;
        }
    }
    return count
}


// true is safe, false is unsafe
fn safe_values(input: Vec<usize>) -> bool {
    let mut safe: bool = true;
    let mut v = vec![];
    // true is increasing, false is decreasing
    let direction: bool;

    for (a, b) in input.into_iter().tuple_windows() {
        v.push((a, b));
    }

    let (x, y) = v[0];
    if y > x {
        // increasing
        direction = true;
    } else {
        // decreasing
        direction = false;
    }

    for (x, y) in v {
        if direction {
            // increasing
            let abs_diff = y.abs_diff(x);
            //dbg!("increasing", x, y, y > x && y.abs_diff(x) <= 3);
            if y > x && 1 <= abs_diff && abs_diff <= 3 {
                safe = safe && true;
            } else {
                safe = false;
            }
        } else {
            //dbg!("decreasing", x, y, x > y && x.abs_diff(y) <= 3);
            let abs_diff = x.abs_diff(y);
            if x > y && 1 <= abs_diff && abs_diff <= 3 {
                safe = safe && true;
            } else {
                safe = false;
            }
        }
    }
    safe
}

fn dampener_safe(input: Vec<usize>) -> bool {
    let mut values = input.clone();
    // true is increasing, false is decreasing
    let direction: bool = values[0] < values[1];

    let mut prev_val: usize = values[0];
    let mut prev_idx: Option<usize> = None;

    for (idx, n) in values.iter().enumerate() {
        if prev_idx.is_some() {
            if direction { // increasing
                if n < &prev_val {
                    values.remove(idx);
                    break
                } else {
                    let abs_diff = n.abs_diff(prev_val);
                    if abs_diff == 0 || abs_diff >= 4 {
                        values.remove(idx);
                        break
                    }
                }
            } else { // decreasing
                if n > &prev_val {
                    values.remove(idx);
                    break
                } else {
                    let abs_diff = prev_val.abs_diff(*n);
                    if abs_diff == 0 || abs_diff >= 4 {
                        values.remove(idx);
                        break
                    }
                }
            }
        }

        prev_val = *n;
        prev_idx = Some(idx);
    }

    safe_values(values)
}

fn parse_nums(input: &str) -> Vec<usize> {
    let parts = input.split(" ");
    let mut nums: Vec<usize> = vec![];

    for part in parts {
        let parsed = part.parse().unwrap_or_else(|_| panic!("AoC wouldn't trick us like that!"));
        nums.push(parsed);
    }

    nums
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let lines = parse_lines_from_str(input);

        assert_eq!(part_1_safe_count(lines), 2);
    }

    #[test]
    fn safe_level_test() {
        // 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
        // 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
        // 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
        // 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
        // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
        // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
        assert_eq!(safe_values(parse_nums("7 6 4 2 1")), true);
        assert_eq!(safe_values(parse_nums("1 2 7 8 9")), false);
        assert_eq!(safe_values(parse_nums("9 7 6 2 1")), false);
        assert_eq!(safe_values(parse_nums("1 3 2 4 5")), false);
        assert_eq!(safe_values(parse_nums("8 6 4 4 1")), false);
    }

    #[test]
    fn part_2_dampener_safe_test() {
        assert_eq!(dampener_safe(parse_nums("7 6 4 2 1")), true, "7 6 4 2 1: Safe without removing any level.");
        assert_eq!(dampener_safe(parse_nums("1 2 7 8 9")), false, "1 2 7 8 9: Unsafe regardless of which level is removed.");
        assert_eq!(dampener_safe(parse_nums("9 7 6 2 1")), false, "9 7 6 2 1: Unsafe regardless of which level is removed.");
        assert_eq!(dampener_safe(parse_nums("1 3 2 4 5")), true, "1 3 2 4 5: Safe by removing the second level, 3.");
        assert_eq!(dampener_safe(parse_nums("8 6 4 4 1")), true, "8 6 4 4 1: Safe by removing the third level, 4.");
        assert_eq!(dampener_safe(parse_nums("1 3 6 7 9")), true, "1 3 6 7 9: Safe without removing any level.");

        // My own examples
        assert_eq!(dampener_safe(parse_nums("1 3 7 4 6")), true, "1 3 7 4 6: Safe from removing the jump from 3 to 7.");
        assert_eq!(dampener_safe(parse_nums("1 3 4 1 5")), true, "1 3 4 1 5: Safe from removing the jump from 4 to 1.");
        assert_eq!(dampener_safe(parse_nums("1 3 4 1 4")), false, "1 3 4 1 4: Unsafe -- removing the jump 4 to 1 leaves 4 4");

    }

    #[test]
    fn part_2_example_test() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let lines = parse_lines_from_str(input);

        assert_eq!(part_2_safe_count(lines), 4);
    }


}
