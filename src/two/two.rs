#[derive(Debug)]
pub struct IdRange {
    from: u64,
    to: u64,
}

pub fn parse_input(s: &str) -> Vec<IdRange> {
    s.split(",")
        .map(|s| {
            let (from, to) = s.split_once("-").unwrap();

            let from = from.trim().parse().unwrap();
            let to = to.trim().parse().unwrap();

            IdRange { from, to }
        })
        .collect()
}

// fn is_valid_part_one(s: &str) -> bool {
//     let l = s.len();

//     if l.rem_euclid(2) == 1 {
//         return true;
//     }

//     for i in 0..l.div_euclid(2) {
//         let first = &s[0..i + 1];

//         let last = &s[i + 1..l];

//         if first == last {
//             return false;
//         }

//         // dbg!(s, l, first, last);
//     }

//     true
// }

fn is_valid(s: &str) -> bool {
    let l = s.len();

    let half = l.div_euclid(2);

    for size in (0..half).rev() {
        let pattern = &s[0..size + 1];

        if l.rem_euclid(size + 1) == 0 {
            let mut valid = false;

            for n in 1..l.div_euclid(size + 1) {
                let start_index = n * (size + 1);

                let compare = &s[start_index..start_index + size + 1];

                if pattern != compare {
                    valid = true;
                    break;
                }
            }

            if !valid {
                return false;
            }
        }
    }

    true
}

pub fn process(ranges: Vec<IdRange>) {
    let mut total = 0u64;

    for range in ranges {
        for i in range.from..range.to + 1 {
            let s = i.to_string();

            let valid = is_valid(&s);

            if !valid {
                dbg!(i);
                total += i;
            }
        }
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // let s = "565653-565659,824824821-824824827";

        let ranges = parse_input(s);

        process(ranges);
    }

    #[test]
    fn test_input() {
        let s = "92916254-92945956,5454498003-5454580069,28-45,4615-7998,4747396917-4747534264,272993-389376,36290651-36423050,177-310,3246326-3418616,48-93,894714-949755,952007-1003147,3-16,632-1029,420-581,585519115-585673174,1041-1698,27443-39304,71589003-71823870,97-142,2790995-2837912,579556301-579617006,653443-674678,1515120817-1515176202,13504-20701,1896-3566,8359-13220,51924-98061,505196-638209,67070129-67263432,694648-751703,8892865662-8892912125";

        let ranges = parse_input(s);

        process(ranges);
    }
}
