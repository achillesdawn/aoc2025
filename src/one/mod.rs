use std::path::Path;

use eyre::Context;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    value: i16,
}

pub fn read_and_parse(path: impl AsRef<Path>) -> eyre::Result<Vec<Instruction>> {
    let s = std::fs::read_to_string(path).wrap_err("could not read to file to string")?;

    let lines: Vec<Instruction> = s
        .lines()
        .map(|s| {
            let value = s[1..].parse::<i16>().unwrap();

            if s.starts_with("L") {
                Instruction {
                    direction: Direction::L,
                    value,
                }
            } else {
                Instruction {
                    direction: Direction::R,
                    value,
                }
            }
        })
        .collect();

    Ok(lines)
}

fn process(lines: Vec<Instruction>) {
    let mut pos = 50i16;

    let mut count = 0;

    for Instruction { direction, value } in lines {
        let rotation = value.rem_euclid(100);

        let passes = value.strict_div(100);
        if passes > 0 {
            println!("adding {passes}");
        }

        count += passes;

        print!(
            "{} rotated {:?} by {} / {} / {passes} -- ",
            pos, direction, value, rotation
        );

        let at_zero = pos == 0;

        match direction {
            Direction::L => {
                
                pos -= rotation;

                if !at_zero && pos < 0 {
                    println!("direction L overflow {pos}");
                    count += 1;
                }
            }
            Direction::R => {
                pos += rotation;

                if !at_zero && pos > 100 {
                    println!("direction R overflow {pos}");
                    count += 1;
                }
            }
        }

        pos = pos.rem_euclid(100);

        println!(" equals {}", pos);

        if pos == 0 {
            println!("one for pos at zero");
            count += 1;
        }
    }

    dbg!(count);
}

pub fn main() {
    let lines = read_and_parse("one_input.txt").unwrap();

    process(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn passes() {
        let lines = read_and_parse("test_one.txt").unwrap();

        process(lines);
    }
}
