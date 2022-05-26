use std::ops::{Add, Div, Mul, Sub};

use eframe::egui::Pos2;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl<T: Clone> From<&T> for Vec2
where
    Vec2: From<T>,
{
    fn from(elem: &T) -> Vec2 {
        Vec2::from(elem.clone())
    }
}

impl<M: Into<f32>, N: Into<f32>> From<(N, M)> for Vec2 {
    fn from((x, y): (N, M)) -> Vec2 {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(vec: Vec2) -> (f32, f32) {
        (vec.x, vec.y)
    }
}

impl From<Vec2> for Pos2 {
    fn from(vec: Vec2) -> Self {
        Pos2 {
            x: vec.x(),
            y: vec.y(),
        }
    }
}

impl From<Pos2> for Vec2 {
    fn from(pos: Pos2) -> Self {
        Vec2 { x: pos.x, y: pos.y }
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    pub fn euclidean_distance(self, other: Vec2) -> f32 {
        self.euclidean_distance_squared(other).sqrt()
    }

    pub fn euclidean_distance_squared(self, other: Vec2) -> f32 {
        (other - self) * (other - self)
    }

    pub fn euclidean_lenght(self) -> f32 {
        (self * self).sqrt()
    }

    pub fn normalise(&self) -> Vec2 {
        *self / self.euclidean_lenght()
    }

    pub fn get_orthogonally_vec(&self) -> Vec2 {
        Vec2 {
            x: self.y(),
            y: -self.x(),
        }
        .normalise()
    }
}

impl Mul for Vec2 {
    type Output = f32;
    fn mul(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i32) -> Vec2 {
        Vec2 {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: rhs.x * self as f32,
            y: rhs.y * self as f32,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x() / rhs,
            y: self.y() / rhs,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        self + (-1 * rhs)
    }
}
