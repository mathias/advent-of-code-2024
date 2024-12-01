#[derive(Debug, PartialEq)]
struct Pair {
    first: u32,
    second: u32
}

fn distances(inputs: Vec<Pair>) -> u32 {
    return 0
}

fn parse_input_str(input: &str) -> Vec<Pair> {
    let split_lines = input.split("\n")
    let mut lines_values: Vec<Pair> = vec![];

    for item in split_lines {
        let parts = item.split(" ");
        let first: u32 = parts[0].into();
        let second: u32 = parts[1].into();
        lines_values.push(Pair { first, second });
    }

    return lines_values;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(first: u32, second: u32) -> Pair {
        return Pair {
            first,
            second
        }
    }

    #[test]
    fn parse_input_str_split() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let output = vec![pair(3, 4), pair(4, 3), pair(2, 5), pair(1, 3), pair(3, 9), pair(3, 3)];

        assert_eq!(parse_input_str(input), output);
    }

    #[test]
    fn example_input() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let lines = parse_input_str(input);

        assert_eq!(distances(lines), 11);
    }
}
