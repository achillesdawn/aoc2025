#[derive(Debug, Clone)]
struct Operation {
    items: Vec<u64>,
    operator: String,
}

pub fn parse_input(s: &str) -> Vec<Operation> {
    let l: Vec<Vec<&str>> = s.lines().map(|l| l.split_whitespace().collect()).collect();

    let n_cols = l.first().unwrap().len();

    let mut result: Vec<Operation> = vec![
        Operation {
            items: Vec::new(),
            operator: "".to_owned()
        };
        n_cols
    ];

    for row in l[..l.len() - 1].iter() {
        for i in 0..n_cols {
            let item = row[i];

            dbg!(item);

            result[i].items.push(item.parse().unwrap());
        }
    }

    for (i, row) in l.last().unwrap().iter().enumerate() {
        result[i].operator = row.to_string();
    }

    result
}

pub fn process(s: &str) {
    let parsed = parse_input(s);

    dbg!(&parsed);

    let mut total = 0u64;

    for item in parsed {
        let r = if item.operator == "*" {
            item.items
                .into_iter()
                .reduce(|acc, e| acc * e)
                .expect("expected reduce to work")
        } else if item.operator == "+" {
            item.items
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
    fn test_one() {
        let s = "123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  ";

        process(s);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/six/input.txt").expect("expected src/six/input.txt");

        process(&s);
    }
}
