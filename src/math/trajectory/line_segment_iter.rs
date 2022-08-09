use crate::{Line2, LineType};

use super::{node::Node, Trajectory};

#[derive(Debug, Clone)]
pub struct LineSegment<'n, N: Node> {
    start: &'n N,
    end: &'n N,
}

impl<'n, N: Node> LineSegment<'n, N> {
    fn new(start: &'n N, end: &'n N) -> LineSegment<'n, N> {
        LineSegment { start, end }
    }

    pub fn start(&self) -> &N {
        self.start
    }

    pub fn end(&self) -> &N {
        self.end
    }
}

impl<'n, N: Node> From<LineSegment<'n, N>> for Line2 {
    fn from(line_segment: LineSegment<'n, N>) -> Self {
        use LineType::LineSegment as LineSegmentType;
        Line2::from_start_and_end(
            line_segment.start().pos(),
            line_segment.end().pos(),
            LineSegmentType,
        )
    }
}

pub struct LineSegmentIter<'d, T: Trajectory> {
    trajectory: &'d T,
    segment_id: usize,
}

impl<'d, T: Trajectory> LineSegmentIter<'d, T> {
    pub fn new(trajectory: &'d T) -> LineSegmentIter<T> {
        LineSegmentIter {
            trajectory,
            segment_id: 0,
        }
    }
}

impl<'d, N: Node + 'd, T: Trajectory<Node = N>> Iterator for LineSegmentIter<'d, T> {
    type Item = LineSegment<'d, N>;
    fn next(&mut self) -> Option<LineSegment<'d, N>> {
        let node_a = self.trajectory.nodes().get(self.segment_id);
        let node_b = self.trajectory.nodes().get(self.segment_id + 1);
        self.segment_id += 1;

        node_a
            .zip(node_b)
            .map(|(start, end)| LineSegment::new(start, end))
    }
}
