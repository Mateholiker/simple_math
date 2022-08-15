use std::f32::{INFINITY, NAN};

use crate::{get_closet_point_on_line_segment, Line2, Vec2};

pub use self::{
    line_segment_iter::LineSegmentIter, node::Node, step_runner::StepRunner,
    tangent_iter::TangentIter,
};

mod line_segment_iter;
mod node;
mod step_runner;
mod tangent_iter;

pub trait Trajectory: Sized {
    type Node: Node;

    fn id(&self) -> u64;

    fn set_id(&mut self, id: u64);

    fn nodes(&self) -> &[Self::Node];

    fn number_of_nodes(&self) -> usize {
        self.nodes().len()
    }

    fn iter_line_segments(&self) -> LineSegmentIter<Self> {
        LineSegmentIter::new(self)
    }

    fn get_step_runner(&self, step_length: f32) -> StepRunner<Self> {
        StepRunner::new(self, step_length)
    }

    fn iter_tangents(&self) -> TangentIter<Self::Node> {
        TangentIter::new(self)
    }

    fn euclidean_distance_to_node<N: Node>(&self, node: &N) -> f32 {
        self.euclidean_distance_to_pos(node.pos())
    }

    fn euclidean_distance_to_pos(&self, pos: Vec2) -> f32 {
        let closest_point = self.closest_point_to_pos(pos, |x| x);
        closest_point.euclidean_distance(pos)
    }

    /// converts the pos and the trajectory node.pos with pos_converter into another coordinate system
    /// calculates the clostet point on the trajectory to pos in the new coordinate system
    /// returns the location vector to this closet point
    /// note: the returnet location vector is still in the new coordinate system
    fn closest_point_to_pos(&self, pos: Vec2, pos_converter: impl Fn(Vec2) -> Vec2) -> Vec2 {
        let pos = pos_converter(pos);
        match self.number_of_nodes() {
            0 => Vec2::new(NAN, NAN),
            1 => pos_converter(self.nodes()[0].pos()),
            _ => {
                let mut min_distance = INFINITY;
                let mut closest_point = self.nodes()[0].pos(); //dummy value
                for line_segment in self.iter_line_segments() {
                    let line_start = pos_converter(line_segment.start().pos());
                    let line_end = pos_converter(line_segment.end().pos());

                    let new_closest_point =
                        get_closet_point_on_line_segment(line_start, line_end, pos);

                    let distance = pos.euclidean_distance(new_closest_point);

                    if distance < min_distance {
                        min_distance = distance;
                        closest_point = new_closest_point;
                    }
                }

                closest_point
            }
        }
    }

    fn euclidean_lenght(&self) -> f32 {
        self.track_length(0, self.number_of_nodes() - 1)
    }

    fn intersections(&self, line: Line2) -> Vec<Vec2> {
        let mut intersections = Vec::new();
        for line_segment in self.iter_line_segments().map(Line2::from) {
            if let Some(intersection) = line.intersection(line_segment) {
                intersections.push(intersection);
            }
        }
        intersections
    }

    ///the length of the partial trajectory form start to end
    fn track_length(&self, start: usize, end: usize) -> f32 {
        assert!(start <= end);
        let mut lenght = 0.0;
        for line_segment in self.iter_line_segments().skip(start).take(end - start) {
            lenght += line_segment.lenght();
        }
        lenght
    }
}
