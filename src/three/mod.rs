pub fn parse_input(s: &str) -> Vec<Vec<u32>> {
    let lines: Vec<Vec<u32>> = s
        .lines()
        .map(|i| i.trim())
        .map(|i| i.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    lines
}

fn line_to_str(line: &[u32]) -> String {
    line.iter()
        .map(|i| i.to_string())
        .reduce(|acc, i| acc + &i)
        .expect("could not convert line to string")
}

fn arg_max(line: &[u32]) -> Option<(usize, u32)> {
    let max = *line
        .iter()
        .max()
        .expect("expected at least one element to calculate arg_max");

    for (i, n) in line.iter().enumerate() {
        if *n == max {
            return Some((i, *n));
        }
    }

    None
}

fn recurse(line: &[u32], start: usize, end: usize, mut result: Vec<u32>) -> Vec<u32> {
    if result.len() == 12 {
        return result;
    }

    let (i, n) = arg_max(&line[start..end]).expect("expected to find a max when calling arg_max");

    result.push(n);

    recurse(line, start + i + 1, end + 1, result)
}

pub fn process(lines: Vec<Vec<u32>>) {
    let mut total = 0u64;

    for line in lines {
        let r = recurse(&line, 0, line.len() - 11, Vec::new());

        let s = line_to_str(&r);

        let n: u64 = s.parse().expect("expected a parsable number");

        total += n;
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_one() {
        let s = r#"987654321111111
            811111111111119
            234234234234278
            818181911112111"#;

        let lines = parse_input(s);

        process(lines);
    }

    #[test]
    fn test_line() {
        let s = read_to_string("src/three/input.txt").expect("could not read input");

        let lines = parse_input(&s);

        let line = lines.get(2).unwrap();

        let line_s = line_to_str(line);

        let r = recurse(line, 0, line.len() - 11, Vec::new());

        let r = line_to_str(&r);

        dbg!(line_s, r);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/three/input.txt").expect("could not read input");

        let lines = parse_input(&s);

        process(lines);
    }
}
