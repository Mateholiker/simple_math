#![feature(const_float_classify)]

mod math;
pub use math::{
    get_closest_point_on_line_segment, get_closest_point_on_line_segment_with_portion,
    line2::{Line2, LineType},
    point_line_segment_distance, point_line_segment_distance_squared,
    rectangle::Rectangle,
    trajectory::{LineSegmentIter, Node, StepRunner, TangentIter, Trajectory},
    vec2::Vec2,
};
