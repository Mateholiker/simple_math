use crate::{math::trajectory::node::Node, Vec2};

use super::Trajectory;

const EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Debug)]
pub struct Step {
    pos_0: Vec2,
    index_0: usize,
    pos_1: Vec2,
    t: f32,
}

impl Step {
    /// Get the steps actual position
    /// step.position() == step.pos_0() * (1 - step.t()) + step.pos_1() * step.t()
    pub fn position(&self) -> Vec2 {
        self.pos_0 * (1.0 - self.t) + self.pos_1 * self.t
    }

    /// Get the step's index 0.
    pub fn index_0(&self) -> usize {
        self.index_0
    }

    /// Get the step's index 1.
    pub fn index_1(&self) -> usize {
        self.index_0 + 1
    }

    /// Get the step's pos 0.
    pub fn pos_0(&self) -> Vec2 {
        self.pos_0
    }

    /// Get the step's pos 1.
    pub fn pos_1(&self) -> Vec2 {
        self.pos_1
    }

    /// Get the step's t.
    /// t is in [0.0, 1.0) and indicates how far we moved from pos_0 to pos_1 in this step
    pub fn t(&self) -> f32 {
        self.t
    }

    fn remaining_lenght(&self) -> f32 {
        self.position().euclidean_distance(self.pos_1)
    }
}

pub struct StepRunner<'d, T: Trajectory> {
    trajectory: &'d T,
    step_length: f32,
    last_step: Option<Step>,
}

impl<'d, T: Trajectory> StepRunner<'d, T> {
    pub fn new(trajectory: &'d T, step_length: f32) -> StepRunner<'d, T> {
        StepRunner {
            trajectory,
            step_length,
            last_step: None,
        }
    }
}

impl<'d, T: Trajectory> Iterator for StepRunner<'d, T> {
    type Item = Step;
    fn next(&mut self) -> Option<Step> {
        if let Some(mut current_step) = self.last_step {
            let mut remaining_step_length = self.step_length;

            while current_step.remaining_lenght() <= remaining_step_length {
                remaining_step_length -= current_step.remaining_lenght();

                let new_index_0 = current_step.index_1();

                let new_step = Step {
                    pos_0: self.trajectory.nodes().get(new_index_0)?.pos(),
                    index_0: new_index_0,
                    pos_1: self.trajectory.nodes().get(new_index_0 + 1)?.pos(),
                    t: 0.0,
                };

                current_step = new_step;
            }
            let line_segment_lenght = current_step.pos_0.euclidean_distance(current_step.pos_1);
            let new_t = remaining_step_length / line_segment_lenght + current_step.t;

            assert!(new_t >= 0.0);

            if new_t > 1.0 + EPSILON {
                dbg!(new_t);
                dbg!(current_step);
            }
            assert!(new_t <= 1.0 + EPSILON);

            let new_step = if new_t >= 1.0 {
                Step {
                    pos_0: self
                        .trajectory
                        .nodes()
                        .get(current_step.index_0() + 1)?
                        .pos(),
                    index_0: current_step.index_0() + 1,
                    pos_1: self
                        .trajectory
                        .nodes()
                        .get(current_step.index_1() + 1)?
                        .pos(),
                    t: 0.0,
                }
            } else {
                Step {
                    t: new_t,
                    ..current_step
                }
            };
            self.last_step = Some(new_step);

            Some(new_step)
        } else {
            let step = Step {
                pos_0: self.trajectory.nodes().get(0)?.pos(),
                pos_1: self.trajectory.nodes().get(1)?.pos(),
                index_0: 0,
                t: 0.0,
            };

            self.last_step = Some(step);

            Some(step)
        }
    }
}
