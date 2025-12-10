use std::str::FromStr;

use itertools::Itertools;

use coord_2d::Coord2D;
use span_1d::Span1D;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(
            AocBufReader::from_string("aoc/src/day_9/data/part_1.txt")
                .map(|x| Coord2D::from_str(&x).unwrap().swap_coords())
                .collect()
        )
    );
    println!(
        "part 2: {}",
        part_2(
            AocBufReader::from_string("aoc/src/day_9/data/part_1.txt")
                .map(|x| Coord2D::from_str(&x).unwrap().swap_coords())
                .collect()
        )
    );
}

fn part_1(tiles: Vec<Coord2D<usize>>) -> usize {
    tiles
        .into_iter()
        .combinations(2)
        .map(|x_y| x_y[0].rectangle_area(&x_y[1]))
        .max()
        .unwrap()
}

/// The tile patch made by the input points has a quite irregular shape.
/// It looks like a circle (with not straight lines) with a large section
/// cut from the middle:
///
///                     ##  ##
///                    ###  ###
///                   ####  ####
///                    ###  #####
///                   ####  ####
///                    ########
///
/// the problem is an awkward amount of large. It's large enough that we probably
/// don't want to brute force it (though I think you could in a few hours). So it'd
/// be nice to come up with a set of criteria that check whether a pair of corners
/// is valid by the new rules.
fn part_2(tiles: Vec<Coord2D<usize>>) -> usize {
    let lines = Line::loop_from_points(tiles.clone());
    if lines
        .iter()
        .combinations(2)
        .any(|v| v[0].is_parallel_with_overlap(v[1]))
    {
        panic!("I didn't expect any of our lines to be parallel and overlap")
    }

    let mut horizontal_lines: Vec<Line> = vec![];
    let mut vertical_lines: Vec<Line> = vec![];
    for line in lines.into_iter() {
        match line.orientation {
            Orientation::Horizontal => {
                horizontal_lines.push(line);
            }
            Orientation::Vertical => {
                vertical_lines.push(line);
            }
        }
    }

    let biggest = tiles
        .iter()
        .combinations(2)
        .filter(|x_y| {
            let x = &x_y[0];
            let y = &x_y[1];
            let min_col = std::cmp::min(x.col, y.col);
            let max_col = std::cmp::max(x.col, y.col);
            let min_row = std::cmp::min(x.row, y.row);
            let max_row = std::cmp::max(x.row, y.row);

            let top = Line::from_points(
                Coord2D::new(min_row, min_col),
                Coord2D::new(min_row, max_col),
            );
            let bottom = Line::from_points(
                Coord2D::new(max_row, min_col),
                Coord2D::new(max_row, max_col),
            );
            let left = Line::from_points(
                Coord2D::new(min_row, min_col),
                Coord2D::new(max_row, min_col),
            );
            let right = Line::from_points(
                Coord2D::new(min_row, max_col),
                Coord2D::new(max_row, max_col),
            );

            for hz in horizontal_lines.iter() {
                if (left.is_some() && hz.intersects(left.as_ref().unwrap()))
                    || (right.is_some() && hz.intersects(right.as_ref().unwrap()))
                    || (right.is_some()
                        && right
                            .as_ref()
                            .unwrap()
                            .vertical_span()
                            .decrement()
                            .contains(hz.start.row)
                        && top.is_some()
                        && top
                            .as_ref()
                            .unwrap()
                            .horizontal_span()
                            .contains(hz.start.col)
                        && top.is_some()
                        && top.as_ref().unwrap().horizontal_span().contains(hz.end.col))
                {
                    return false;
                }
            }

            for vl in vertical_lines.iter() {
                if (top.is_some() && vl.intersects(top.as_ref().unwrap()))
                    || (bottom.is_some() && vl.intersects(bottom.as_ref().unwrap()))
                    || (top.is_some()
                        && top
                            .as_ref()
                            .unwrap()
                            .horizontal_span()
                            .decrement()
                            .contains(vl.start.col)
                        && right.is_some()
                        && right
                            .as_ref()
                            .unwrap()
                            .vertical_span()
                            .contains(vl.start.row)
                        && right.is_some()
                        && right.as_ref().unwrap().vertical_span().contains(vl.end.row))
                {
                    return false;
                }
            }

            for tile in tiles.iter() {
                if left.is_some()
                    && left
                        .as_ref()
                        .unwrap()
                        .vertical_span()
                        .decrement()
                        .contains(tile.row)
                    && top.is_some()
                    && top
                        .as_ref()
                        .unwrap()
                        .horizontal_span()
                        .decrement()
                        .contains(tile.col)
                {
                    return false;
                }
            }

            true
        })
        .max_by_key(|x_y| x_y[0].rectangle_area(x_y[1]))
        .unwrap();

    println!("{:?}", biggest);
    biggest[0].rectangle_area(biggest[1])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Line {
    start: Coord2D<usize>,
    end: Coord2D<usize>,
    orientation: Orientation,
}

impl Line {
    fn new(start: Coord2D<usize>, end: Coord2D<usize>, orientation: Orientation) -> Self {
        Self {
            start,
            end,
            orientation,
        }
    }

    /// for some sanity checking; do these lines share an orientation and
    /// share some coordinates? I don't think our input has lines like this.
    /// Let's check
    fn is_parallel_with_overlap(&self, other: &Self) -> bool {
        match (self.orientation, other.orientation) {
            (Orientation::Horizontal, Orientation::Horizontal) => {
                self.start.row == other.start.row
                    && self.horizontal_span().intersects(&other.horizontal_span())
            }
            (Orientation::Horizontal, Orientation::Vertical) => false,
            (Orientation::Vertical, Orientation::Horizontal) => false,
            (Orientation::Vertical, Orientation::Vertical) => {
                self.start.col == other.start.col
                    && self.vertical_span().intersects(&other.vertical_span())
            }
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        match (self.orientation, other.orientation) {
            (Orientation::Horizontal, Orientation::Horizontal) => false,
            (Orientation::Horizontal, Orientation::Vertical) => {
                other.vertical_span().decrement().contains(self.start.row)
                    && self.horizontal_span().decrement().contains(other.start.col)
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                self.vertical_span().decrement().contains(other.start.row)
                    && other.horizontal_span().decrement().contains(self.start.col)
            }
            (Orientation::Vertical, Orientation::Vertical) => false,
        }
    }

    fn horizontal_span(&self) -> Span1D<usize> {
        if self.start.col >= self.end.col {
            Span1D::from_start_end_inclusive(self.end.col, self.start.col)
        } else {
            Span1D::from_start_end_inclusive(self.start.col, self.end.col)
        }
    }

    fn vertical_span(&self) -> Span1D<usize> {
        if self.start.row >= self.end.row {
            Span1D::from_start_end_inclusive(self.end.row, self.start.row)
        } else {
            Span1D::from_start_end_inclusive(self.start.row, self.end.row)
        }
    }

    fn from_points(start: Coord2D<usize>, end: Coord2D<usize>) -> Option<Self> {
        if start == end {
            return None;
        }

        let orientation = if start.row == end.row {
            Orientation::Horizontal
        } else if start.col == end.col {
            Orientation::Vertical
        } else {
            panic!("wasn't expecting a diagonal line")
        };
        Some(Self::new(start, end, orientation))
    }

    fn loop_from_points(points: Vec<Coord2D<usize>>) -> Vec<Self> {
        let n_points = points.len();
        let mut result: Vec<Self> = points[..(n_points - 1)]
            .iter()
            .zip(points[1..].iter())
            .map(|(start, end)| Self::from_points(start.clone(), end.clone()).unwrap())
            .collect();
        result.push(Self::from_points(points[n_points - 1].clone(), points[0].clone()).unwrap());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",]
                    .into_iter()
                    .map(|x| Coord2D::from_str(x).unwrap())
                    .collect()
            ),
            50
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",]
                    .into_iter()
                    .map(|x| Coord2D::from_str(x).unwrap().swap_coords())
                    .collect()
            ),
            24
        )
    }

    #[test]
    fn test_intersects() {
        //   ^
        //   |
        // <-+>
        //   v
        let hz = Line::from_points(Coord2D::new(2, 0), Coord2D::new(2, 3)).unwrap();
        let vl = Line::from_points(Coord2D::new(0, 2), Coord2D::new(3, 2)).unwrap();
        assert!(hz.intersects(&vl));
        assert!(vl.intersects(&hz));

        //
        //
        // <-+>
        //   |
        //   v
        let hz = Line::from_points(Coord2D::new(2, 0), Coord2D::new(2, 3)).unwrap();
        let vl = Line::from_points(Coord2D::new(2, 2), Coord2D::new(3, 2)).unwrap();
        assert!(!hz.intersects(&vl));
        assert!(!vl.intersects(&hz));
    }

    #[test]
    fn test_debug_case() {
        let hz = Line {
            start: Coord2D { row: 5, col: 9 },
            end: Coord2D { row: 5, col: 2 },
            orientation: Orientation::Horizontal,
        };
        let right = Line {
            start: Coord2D { row: 3, col: 9 },
            end: Coord2D { row: 5, col: 9 },
            orientation: Orientation::Vertical,
        };

        assert!(!hz.intersects(&right));
    }
}
