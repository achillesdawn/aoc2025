use std::{collections::HashSet, ops::Range};

pub fn parse_input(s: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let lines: Vec<&str> = s.lines().map(|line| line.trim()).collect();

    let br = lines
        .iter()
        .position(|i| i.is_empty())
        .expect("expected empty line");

    let (first, second) = lines.split_at(br);

    let ranges: Vec<(u64, u64)> = first
        .iter()
        .map(|i| {
            let (first, second) = i.split_once("-").expect("expected a dash (-)");

            (first.parse().unwrap(), second.parse().unwrap())
        })
        .collect();

    let ranges: Vec<Range<u64>> = ranges.into_iter().map(|(s, e)| s..(e + 1)).collect();

    let nums: Vec<u64> = second[1..]
        .iter()
        .map(|i| i.parse().expect("could not parse to num"))
        .collect();

    (ranges, nums)
}

pub fn process(ranges: Vec<Range<u64>>, nums: Vec<u64>) {
    let mut total = 0u32;

    for num in nums.into_iter() {
        'inner: for range in ranges.iter() {
            if range.contains(&num) {
                total += 1;
                break 'inner;
            }
        }
    }

    dbg!(total);
}

pub fn synthesize(mut ranges: Vec<Range<u64>>) {
    ranges.sort_by_key(|i| i.start);

    let mut synthesized = Vec::new();

    let mut pruned_idxs = HashSet::new();

    for (mut idx, range) in ranges.iter().enumerate() {
        if pruned_idxs.contains(&idx) {
            continue;
        }

        let mut r = range.clone();

        println!("start {:?}", r);

        'inner: loop {
            let Some(next) = ranges.get(idx + 1) else {
                synthesized.push(r);
                break;
            };

            println!("next {:?}", next);

            if next.contains(&(r.end - 1)) || r.contains(&next.start) {
                println!("extending range to {}..{}", r.start, next.end);

                r.end = next.end;

                pruned_idxs.insert(idx + 1);

                idx += 1;
            } else {
                println!("end {}..{}\n\n", r.start, r.end);

                synthesized.push(r);

                break 'inner;
            }
        }
    }

    let mut total = 0usize;

    dbg!(&synthesized);

    for r in synthesized {
        let (lower, _) = r.size_hint();

        total += lower;
    }

    dbg!(total);
}

// fn process(s: &str) {}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_one() {
        let s = "3-5
            10-14
            16-20
            12-18
            5-5
            12-18
            12-19
            12-20
            11-14
            11-20
            3-4
            10-10
            10-11
            19-20
            
            1
            5
            8
            11
            17
            32";

        let (ranges, _) = parse_input(s);

        synthesize(ranges);
    }

    #[test]
    fn test_input() {
        //  324092551403442
        //  324092551403535
        //  340290417971441
        //  356565622565127
        //  386407253620082 too high
        //  426810194193389
        //  426810194193504
        let s = read_to_string("src/five/input.txt").expect("expected input file");

        let (ranges, _) = parse_input(&s);

        synthesize(ranges);
    }
}
