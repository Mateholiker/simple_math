use std::fmt::Debug;

use crate::Vec2;

pub trait Node: Debug + Clone {
    fn pos(&self) -> Vec2;

    fn euclidean_distance<N: Node>(&self, other: &N) -> f32 {
        self.pos().euclidean_distance(other.pos())
    }

    fn euclidean_distance_to_pos(&self, other: Vec2) -> f32 {
        self.pos().euclidean_distance(other)
    }
}
