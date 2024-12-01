use crate::util;

fn distances(inputs: Vec<String>) -> u32 {
    let mut first_list: Vec<u32> = vec![];
    let mut second_list: Vec<u32> = vec![];

    for line in inputs {
        let parts: Vec<&str> = line.split(" ").collect();
        first_list.push(parts[0].parse().expect("Must be an integer"));
        second_list.push(parts[1].parse().expect("Must be an integer"));

    }

    first_list.into_iter().sum()
}

fn parse_lines_from_str(input: &str) -> Vec<String> {
    input.split("\n").into_iter().map(|s| s.to_string() ).collect::<Vec<String>>()
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
