use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StrOperation {
    nums: HashMap<usize, String>,
    operator: String,
}

#[derive(Debug)]
pub struct Operation {
    nums: Vec<u64>,
    operator: String,
}

impl From<StrOperation> for Operation {
    fn from(value: StrOperation) -> Self {
        let nums: Vec<_> = value
            .nums
            .values()
            .map(|v| v.trim().parse().unwrap())
            .collect();

        Self {
            nums,
            operator: value.operator,
        }
    }
}

fn calculate_distances(last: &str) -> Vec<usize> {
    let distances: Vec<_> = last
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .collect();

    let mut sizes = Vec::new();

    for (idx, current) in distances.iter().enumerate() {
        let Some(next) = distances.get(idx + 1) else {
            sizes.push(last.len() + 1 - current.0);

            break;
        };

        let size = next.0 - current.0;

        sizes.push(size);
    }

    sizes
}

pub fn parse_input(s: &str) -> Vec<Operation> {
    let lines: Vec<&str> = s.lines().collect();

    let last = lines.last().unwrap();

    let signs: Vec<&str> = last.split_whitespace().collect();

    let sizes = calculate_distances(last);

    let mut operations = vec![
        StrOperation {
            nums: HashMap::new(),
            operator: String::new()
        };
        sizes.len()
    ];

    for (i, op) in signs.iter().enumerate() {
        let operation = operations.get_mut(i).unwrap();

        operation.operator = op.to_string();
    }

    for line in &lines[..lines.len() - 1] {
        let mut idx = 0;

        for (i, size) in sizes.iter().enumerate() {
            let operation = operations.get_mut(i).unwrap();

            let v = &line[idx..idx + size - 1];

            for (n, c) in v.chars().enumerate() {
                let entry = operation.nums.entry(n).or_insert(String::new());

                entry.push(c);
            }

            idx += size;
        }
    }

    operations.into_iter().map(|v| v.into()).collect()
}

pub fn process(s: &str) {
    let parsed = parse_input(s);

    let mut total = 0u64;

    for item in parsed {
        let r = if item.operator == "*" {
            item.nums
                .into_iter()
                .reduce(|acc, e| acc * e)
                .expect("expected reduce to work")
        } else if item.operator == "+" {
            item.nums
                .into_iter()
                .reduce(|acc, e| acc + e)
                .expect("expected reduce to work")
        } else {
            panic!("unexpected operator");
        };

        total += r;
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_one() -> eyre::Result<()> {
        let s = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        process(s);

        Ok(())
    }

    #[test]
    fn test_validate() {
        let s = read_to_string("src/six/input.txt").expect("expected src/six/input.txt");

        let operations = parse_input(&s);

        let mut iter = operations.iter();

        let second = iter.nth(2).unwrap();
        let ten = iter.nth(10).unwrap();
        let twenty = iter.nth(20).unwrap();
        let last = iter.last().unwrap();

        dbg!(second, ten, twenty, last);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/six/input.txt").expect("expected src/six/input.txt");

        process(&s);
    }
}
