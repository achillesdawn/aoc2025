use std::ops::Range;

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

    let ranges: Vec<Range<u64>> = ranges.into_iter().map(|(s, e)| s..e).collect();

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
            
            1
            5
            8
            11
            17
            32";

        let (ranges, nums) = parse_input(s);

        process(ranges, nums);
    }

    #[test]
    fn test_input() {
        let s = read_to_string("src/five/input.txt").expect("expected input file");

        let (ranges, nums) = parse_input(&s);

        process(ranges, nums);
    }
}
