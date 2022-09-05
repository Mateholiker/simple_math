pub mod line2;
pub mod rectangle;
pub mod trajectory;
pub mod vec2;

use vec2::Vec2;

pub fn get_closest_point_on_line_segment_with_portion(
    line_start: Vec2,
    line_end: Vec2,
    point: Vec2,
) -> (Vec2, f32) {
    //the following algorithm is from https://stackoverflow.com/questions/849211/shortest-euclidean_distance-between-a-point-and-a-line-segment

    let length_squared = line_end.euclidean_distance_squared(line_start);

    if length_squared.abs() < 0.000001 {
        //if line_start == line_end
        (line_start, 0.0)
    } else {
        //line_start + (line_end - line_start) * t is our line segment
        // we calculate t so line_start + (line_end - line_start) * t is the closed point on the segment
        //therefore we clamp t from [0,1]
        let t = f32::max(
            0.0,
            f32::min(
                1.0,
                ((point - line_start) * (line_end - line_start)) / length_squared,
            ),
        );

        (line_start + (line_end - line_start) * t, t)
    }
}

pub fn get_closest_point_on_line_segment(line_start: Vec2, line_end: Vec2, point: Vec2) -> Vec2 {
    get_closest_point_on_line_segment_with_portion(line_start, line_end, point).0
}

pub fn point_line_segment_distance(line_start: Vec2, line_end: Vec2, point: Vec2) -> f32 {
    let projection = get_closest_point_on_line_segment(line_start, line_end, point);

    point.euclidean_distance(projection)
}

pub fn point_line_segment_distance_squared(line_start: Vec2, line_end: Vec2, point: Vec2) -> f32 {
    let projection = get_closest_point_on_line_segment(line_start, line_end, point);

    point.euclidean_distance_squared(projection)
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        use std::cmp::Ordering;
        match $x.partial_cmp(&y) {
            Some(Ordering::Less) => y,
            Some(Ordering::Equal) => $x,
            Some(Ordering::Greater) => $x,
            None => {
                panic!("Could not compare {} and {}", $x, y)
            }
        }
    }}
}

#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        use std::cmp::Ordering;
        match $x.partial_cmp(&y) {
            Some(Ordering::Less) => $x,
            Some(Ordering::Equal) => $x,
            Some(Ordering::Greater) => y,
            None => {
                panic!("Could not compare {} and {}", $x, y)
            }
        }
    }}
}
