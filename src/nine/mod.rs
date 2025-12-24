use std::collections::HashSet;

use geo::{Area, Intersects, Line, Point, Rect, Scale};
use tracing::info;

pub fn parse(s: &str) -> Vec<Point> {
    s.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(first, second)| (first.trim(), second.trim()))
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()).into())
        .collect()
}

fn calc_area(pos: &Point, other: &Point) -> f64 {
    let dim_a = (pos.y() - other.y()).abs() + 1.;
    let dim_b = (other.x() - pos.x()).abs() + 1.;

    dim_a * dim_b
}

#[derive(Debug)]
pub struct RectArea {
    area: f64,
    points: (Point, Point),
}

fn create_lines(points: &[Point]) -> Vec<Line> {
    let mut lines = Vec::new();

    for (idx, point) in points.iter().enumerate() {
        if let Some(next) = points.get(idx + 1) {
            let line = Line::new(*point, *next);

            lines.push(line);
        }
    }

    let first = points.first().unwrap();
    let last = points.last().unwrap();

    lines.push(Line::new(*last, *first));

    lines
}

pub fn main(points: Vec<Point>) {
    let lines = create_lines(&points);

    let mut checked = HashSet::new();

    let mut areas = Vec::new();

    for (first_idx, first) in points.iter().enumerate() {
        'outer: for (second_idx, second) in points.iter().enumerate() {
            if first_idx == second_idx || checked.contains(&second_idx) {
                continue;
            }

            let rect = Rect::new(*first, *second);
            let bbox = rect.scale(0.99999999);

            for line in lines.iter() {
                if bbox.intersects(line) {
                    continue 'outer;
                }
            }

            // let area = rect.signed_area();

            let area = calc_area(first, second);

            areas.push(RectArea {
                area,
                points: (*first, *second),
            });
        }

        checked.insert(first_idx);
    }

    areas.sort_by(|a, b| a.area.total_cmp(&b.area));

    info!(?areas, "done");
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use geo::Scale;
    use tracing::debug;

    use super::*;

    fn init_tracing() {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
                "%H:%M:%S%.3f".to_owned(),
            ))
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

        let points = parse(s);

        main(points);
    }

    #[test]
    fn intersections() {
        init_tracing();

        let point_1 = Point::new(11., 1.);
        let point_2 = Point::new(2., 3.);

        let mut rect = Rect::new(point_1, point_2);

        rect = rect.scale(0.99);

        let intersect_lines = [((7., 1.), (7., 3.)), ((3., 0.), (3., 2.))];

        for (p1, p2) in intersect_lines {
            let line: Line = Line::new(Point::new(p1.0, p1.1), Point::new(p2.0, p2.1));

            assert!(rect.intersects(&line));
        }

        let no_intersect_lines = [
            ((5., 0.), (1., 0.)),
            ((1., 1.), (2., 1.)),
            ((1., 0.), (5., 1.)),
        ];

        for (p1, p2) in no_intersect_lines {
            let line: Line = Line::new(Point::new(p1.0, p1.1), Point::new(p2.0, p2.1));

            debug!(?line);

            assert!(!rect.intersects(&line));
        }
    }

    #[test]
    fn with_input() {
        init_tracing();

        let s = read_to_string("src/nine/input.txt").unwrap();

        let points = parse(&s);

        main(points);
    }
}
