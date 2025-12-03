pub fn parse_input(s: &str) -> Vec<Vec<u32>> {
    let lines: Vec<Vec<u32>> = s
        .lines()
        .map(|i| i.trim())
        .map(|i| i.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    lines
}

fn line_as_str(line: &[u32]) {
    let s = line
        .iter()
        .map(|i| i.to_string())
        .reduce(|acc, i| acc + &i)
        .expect("could not convert line to string");

    dbg!(s);
}

// pub fn process_part_one(lines: Vec<Vec<u32>>) {
//     let mut total = 0u32;

//     for line in lines {
//         let len = line.len();
//         let mut max_indices: Vec<_> = Vec::new();

//         let max = *line[..len - 1].iter().max().expect("expected a max number");

//         for (index, value) in line.iter().enumerate() {
//             if *value == max {
//                 max_indices.push((index, value));
//             }
//         }

//         line_as_str(&line);

//         let mut nums = Vec::new();

//         for (index, value) in max_indices.iter() {
//             dbg!(index, max);
//             let slice = &line[*index + 1..];

//             let Some(local_max) = slice.iter().max() else {
//                 continue;
//             };

//             let num = *value * 10 + *local_max;

//             nums.push(num);
//         }

//         let result = nums.iter().max().expect("expected at least one num");

//         total += result;
//     }

//     dbg!(total);
// }

fn process_line(line: &[u32]) -> u32 {
    let len = line.len();

    let mut max_indices: Vec<_> = Vec::new();

    let max = *line[..len - 1].iter().max().expect("expected a max number");

    for (index, value) in line.iter().enumerate() {
        if *value == max {
            max_indices.push((index, value));
        }
    }

    line_as_str(line);

    let mut nums = Vec::new();

    for (index, value) in max_indices.iter() {
        dbg!(index, max);
        let slice = &line[*index + 1..];

        let Some(local_max) = slice.iter().max() else {
            continue;
        };

        let num = *value * 10 + *local_max;

        nums.push(num);
    }

    let result = nums.iter().max().expect("expected at least one num");

    *result
}

pub fn process(lines: Vec<Vec<u32>>) {
    let mut total = 0u32;

    for line in lines {
        let result = process_line(&line);

        total += result;
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_one() {
        let s = r#"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"#;

        let lines = parse_input(s);

        process(lines);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/three/input.txt").expect("could not read input");

        let lines = parse_input(&s);

        process(lines);
    }
}
