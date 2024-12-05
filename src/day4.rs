//use regex::Regex;
use crate::util::parse_file;

const PATH: &str = "inputs/day4.txt";

fn part_1() {
    let lines = parse_file(PATH);

    println!("{}", xmas_count(lines));
}

fn part_2() {
    let lines = parse_file(PATH);

    println!("{}", part_2_count(lines));
}

fn xmas_count(input: Vec<String>) -> usize {
    let mut count: usize = 0;

    count += count_left_right(input.clone());
    // diagonals tricky business
    count += count_diagonals(input.clone());

    // vertical counts
    count += count_vertical_matches(input.clone());

    return count
}

fn count_left_right(input: Vec<String>) -> usize {
    let mut count: usize = 0;

    for line in input {
        if line.matches("XMAS").collect::<Vec<&str>>().len() > 0 {
            let matches = line.matches("XMAS").collect::<Vec<&str>>();
            count += matches.len();
        }
        if line.matches("SAMX").collect::<Vec<&str>>().len() > 0 {
            let matches = line.matches("SAMX").collect::<Vec<&str>>();
            count += matches.len();
        }
    }


    return count
}

fn count_vertical_matches(input: Vec<String>) -> usize {
    let matrix: Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect() ).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated: Vec<Vec<char>> = vec![vec!['.'; rows]; cols];
    let mut rotated_strings: Vec<String> = vec![];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][i] = matrix[i][j];
        }
    }

    for row in 0..rotated.len() {
        rotated_strings.push(rotated[row].clone().into_iter().collect::<String>());
    }
    dbg!(rotated_strings.clone());

    count_left_right(rotated_strings)
}


fn check_diagonal(lines: Vec<Vec<char>>, target_chars: &Vec<char>, row: usize, col: usize, row_step: isize, col_step: isize) -> bool {
    for i in 0..target_chars.len() {
        let r: usize = ((i as isize * row_step) + row as isize) as usize;
        let c: usize = ((i as isize * col_step) + col as isize) as usize;
        if r >= lines.len() || c >= lines[r].len() || lines[r][c] != target_chars[i] {
            return false;
        }
    }
    true
}

fn count_diagonals(input: Vec<String>) -> usize {
    let matrix: Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect() ).collect();
    let target_chars: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if check_diagonal(matrix.clone(), &target_chars, row, col, 1, 1) {
                count += 1;
            }

            if check_diagonal(matrix.clone(), &target_chars, row, col, 1, -1) {
                count += 1;
            }

            if check_diagonal(matrix.clone(), &target_chars, row, col, -1, -1) {
                count += 1;
            }

            if check_diagonal(matrix.clone(), &target_chars, row, col, -1, 1) {
                count += 1;
            }
        }
    }

    return count
}

fn rotate_chars_45_degrees(input: Vec<String>) -> Vec<String> {
    let mut input_chars: Vec<Vec<char>> = vec![];
    let mut chars: Vec<char>;

    for line in input {
        chars = line.chars().collect();
        input_chars.push(chars);
    }

    let mut rotated_chars: Vec<Vec<char>> = vec![];
    let mut ctr = 0;
    let width = input_chars[0].len(); // width
    let height = input_chars.len(); // height
    let mut lst: Vec<char>;
    while ctr < 2 * width - 1 {
        lst = vec![];
        for i in 0..height {
            for j in 0..width {
                if i + j == ctr {
                    lst.push(input_chars[i][j])
                }
            }
        }
        rotated_chars.push(lst);
        ctr += 1;
    }
    rotated_chars.into_iter().map(|v| v.into_iter().collect() ).collect()
}

fn part_2_count(input: Vec<String>) -> usize {
    let matrix: Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect() ).collect();
    let target_chars: Vec<char> = "MAS".chars().collect();
    let targ_len_idx = target_chars.len() - 1;
    let mut count = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            // check_diagonal(lines: , target_chars: , row: usize, col: usize, row_step: isize, col_step: isize) -> bool {
            //
            // M.S
            // .A.
            // M.S
            if check_diagonal(matrix.clone(), &target_chars, row, col, 1, 1) &&
                check_diagonal(matrix.clone(), &target_chars, row + targ_len_idx, col, -1, 1) {
                count += 1;
            }

            // M.M
            // .A.
            // S.S
            if check_diagonal(matrix.clone(), &target_chars, row, col, 1, 1) &&
                check_diagonal(matrix.clone(), &target_chars, row, col + targ_len_idx, 1, -1) {
                count += 1;
            }

            //S.S
            //.A.
            //M.M";
            if check_diagonal(matrix.clone(), &target_chars, row, col, -1, 1) &&
                check_diagonal(matrix.clone(), &target_chars, row, col + targ_len_idx, -1, -1) {
                count += 1;
            }

            //S.M
            //.A.
            //S.M
            if check_diagonal(matrix.clone(), &target_chars, row, col, 1, -1) &&
                check_diagonal(matrix.clone(), &target_chars, row + targ_len_idx, col, -1, -1) {
                count += 1;
            }
        }
    }

    return count
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
    //use pretty_assertions::assert_eq;
    use crate::util::parse_lines_from_str;

    #[test]
    fn find_xmas_multiple_same_line_test() {
        let input = "..XMAS.XMAS.";

        let parsed_input = parse_lines_from_str(input);
        let count = xmas_count(parsed_input);

        assert_eq!(2, count);
    }

    #[test]
    fn rotate_chars_45_degrees_test() {
        let input = "\
...X
..M.
.A..
S...";
        let expected: Vec<String> = vec![
".".to_string(),
"..".to_string(),
"...".to_string(),
"XMAS".to_string(),
"...".to_string(),
"..".to_string(),
".".to_string()
        ];

        let parsed_input = parse_lines_from_str(input);
        assert_eq!(expected, rotate_chars_45_degrees(parsed_input));
    }

    #[test]
    fn find_xmas_diagonals_test() {
        let input = "\
......X..
.....M...
....A....
...S.....
X........
.M.......
..A......
...S.....
S........
.A.......
..M......
...X.....
......S..
.....A...
....M....
...X.....";

        let parsed_input = parse_lines_from_str(input);
        let count = count_diagonals(parsed_input);
        assert_eq!(4, count);
    }

    #[test]
    fn find_xmas_verticals_test() {
        let input = "\
X....
M...S
A...A
S...M
....X";

        let parsed_input = parse_lines_from_str(input);
        let count = count_vertical_matches(parsed_input);

        assert_eq!(2, count);
    }

    #[test]
    fn find_xmas_test_simple() {
        let input = "\
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

        let parsed_input = parse_lines_from_str(input);
        let count = xmas_count(parsed_input);

        assert_eq!(18, count);
    }

    #[test]
    fn find_xmas_test_complex() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let parsed_input = parse_lines_from_str(input);
        let count = xmas_count(parsed_input);

        assert_eq!(18, count);
    }

    #[test]
    fn part_2_count_test() {
        let input = "\
M.S
.A.
M.S";

        let parsed_input = parse_lines_from_str(input);
        let count = part_2_count(parsed_input);

        assert_eq!(1, count);

        let input = "\
M.M
.A.
S.S";

        let parsed_input = parse_lines_from_str(input);
        let count = part_2_count(parsed_input);

        assert_eq!(1, count);

        let input = "\
S.S
.A.
M.M";

        let parsed_input = parse_lines_from_str(input);
        let count = part_2_count(parsed_input);

        assert_eq!(1, count);

        let input = "\
S.M
.A.
S.M";

        let parsed_input = parse_lines_from_str(input);
        let count = part_2_count(parsed_input);

        assert_eq!(1, count);
    }

    #[test]
    fn part_2_count_complex_test() {
        let input = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let parsed_input = parse_lines_from_str(input);
        let count = part_2_count(parsed_input);

        assert_eq!(9, count);
    }
}
