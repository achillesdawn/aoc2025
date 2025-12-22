use geo::{Line, LineIntersection, Point, line_intersection};

pub fn parse(s: &str) -> Vec<Point> {
    s.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(first, second)| (first.trim(), second.trim()))
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()).into())
        .collect()
}

fn intersection(
    line1_point1: Point,
    line1_point2: Point,
    line2_point1: Point,
    line2_point2: Point,
) -> Option<LineIntersection<f64>> {
    let line1 = Line::new(line1_point1, line1_point2);

    let line_2 = Line::new(line2_point1, line2_point2);

    line_intersection::line_intersection(line1, line_2)
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use tracing::info;

    use super::*;

    fn init_tracing() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    }

    #[test]
    fn case_one() {
        init_tracing();

        let s = "7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3";

        let positions = parse(s);

        info!(?positions);
    }

    #[test]
    fn intersections() {
        init_tracing();

        intersection(
            (11., 1.).into(),
            (2., 3.).into(),
            (7., 1.).into(),
            (7., 3.).into(),
        );
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/nine/input.txt").unwrap();

        let positions = parse(&s);
    }
}
