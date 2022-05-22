pub mod line2;
pub mod rectangle;
pub mod vec2;

use vec2::Vec2;

pub fn get_closet_point_on_line_segment(line_start: Vec2, line_end: Vec2, point: Vec2) -> Vec2 {
    //the following algorithm is from https://stackoverflow.com/questions/849211/shortest-euclidean_distance-between-a-point-and-a-line-segment

    let length_squared = line_end.euclidean_distance_squared(line_start);

    if length_squared.abs() < 0.000001 {
        //if line_start == line_end
        line_start
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

        line_start + (line_end - line_start) * t
    }
}

pub fn point_line_segment_distance(line_start: Vec2, line_end: Vec2, point: Vec2) -> f32 {
    let projection = get_closet_point_on_line_segment(line_start, line_end, point);

    point.euclidean_distance(projection)
}

pub fn point_line_segment_distance_squared(line_start: Vec2, line_end: Vec2, point: Vec2) -> f32 {
    let projection = get_closet_point_on_line_segment(line_start, line_end, point);

    point.euclidean_distance_squared(projection)
}
