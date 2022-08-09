use crate::{Line2, LineType, Vec2};

use super::{node::Node, Trajectory};

pub struct TangentIter<'d, N: Node> {
    line_segment_buffer: Option<Vec2>,
    line_segment_iter: Box<(dyn Iterator<Item = Vec2> + 'd)>,
    node_iter: Box<(dyn Iterator<Item = &'d N> + 'd)>,
}

impl<'d, N: Node> TangentIter<'d, N> {
    pub fn new<T: Trajectory<Node = N>>(trajectory: &'d T) -> TangentIter<'d, N> {
        let line_segment_iter = trajectory
            .iter_line_segments()
            .map(|line_segment| line_segment.end().pos() - line_segment.start().pos())
            .fuse();

        TangentIter {
            line_segment_buffer: None,
            line_segment_iter: Box::new(line_segment_iter),
            node_iter: Box::new(trajectory.nodes().iter()),
        }
    }
}

impl<'d, N: Node> Iterator for TangentIter<'d, N> {
    type Item = Line2;
    fn next(&mut self) -> Option<Line2> {
        use LineType::Line;
        let line_segment = self.line_segment_iter.next();
        let node = self.node_iter.next()?;

        let gradient = match (self.line_segment_buffer, line_segment) {
            (Some(first), Some(second)) => {
                let first = first.normalise();
                let second = second.normalise();
                Some((first + second).normalise())
            }
            (None, Some(second)) => Some(second.normalise()),
            (Some(first), None) => Some(first.normalise()),
            (None, None) => None,
        };

        self.line_segment_buffer = line_segment;

        Some(Line2::from_support_point_and_vector(
            node.pos(),
            gradient?,
            Line,
        ))
    }
}
