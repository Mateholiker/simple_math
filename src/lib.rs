mod math;
pub use math::{
    get_closet_point_on_line_segment,
    line2::{Line2, LineType},
    point_line_segment_distance, point_line_segment_distance_squared,
    rectangle::Rectangle,
    trajectory::{LineSegmentIter, Node, StepRunner, TangentIter, Trajectory},
    vec2::Vec2,
};
