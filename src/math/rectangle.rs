use eframe::egui::Rect;

use super::Vec2;

#[derive(Clone, Copy, Default, Debug)]
pub struct Rectangle {
    min: Vec2,
    max: Vec2,
}

impl Rectangle {
    pub fn new(corner_a: Vec2, corner_b: Vec2) -> Rectangle {
        let (min_x, max_x) = if corner_a.x() < corner_b.x() {
            (corner_a.x(), corner_b.x())
        } else {
            (corner_b.x(), corner_a.x())
        };
        let (min_y, max_y) = if corner_a.y() < corner_b.y() {
            (corner_a.y(), corner_b.y())
        } else {
            (corner_b.y(), corner_a.y())
        };

        let min = Vec2::new(min_x, min_y);
        let max = Vec2::new(max_x, max_y);

        Rectangle { min, max }
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn top(&self) -> f32 {
        self.max.y()
    }

    pub fn bottom(&self) -> f32 {
        self.min.y()
    }

    pub fn right(&self) -> f32 {
        self.max.x()
    }

    pub fn left(&self) -> f32 {
        self.min.x()
    }

    pub fn width(&self) -> f32 {
        self.max().x() - self.min().x()
    }

    pub fn height(&self) -> f32 {
        self.max().y() - self.min().y()
    }

    ///shrinks the rectangle in each direction, keeping the center
    pub fn shrink(self, amount: f32) -> Rectangle {
        let min = self.min + Vec2::new(amount, amount);
        let max = self.max - Vec2::new(amount, amount);
        Rectangle { min, max }
    }

    pub fn extend_with_vec2(&mut self, vec: Vec2) {
        let min_x = self.min.x().min(vec.x());
        let min_y = self.min.y().min(vec.y());

        let max_x = self.max.x().max(vec.x());
        let max_y = self.max.y().max(vec.y());

        self.min = Vec2::from((min_x, min_y));
        self.max = Vec2::from((max_x, max_y));
    }

    pub fn extend_with_rectangle(&mut self, other: Rectangle) {
        self.extend_with_vec2(other.min);
        self.extend_with_vec2(other.max);
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        self.min.x() <= pos.x()
            && self.max.x() >= pos.x()
            && self.min.y() <= pos.y()
            && self.max.y() >= pos.y()
    }
}

impl From<Rectangle> for Rect {
    fn from(rectangle: Rectangle) -> Self {
        Rect {
            min: rectangle.min().into(),
            max: rectangle.max().into(),
        }
    }
}

impl From<&Rectangle> for Rect {
    fn from(elem: &Rectangle) -> Rect {
        Rect::from(*elem)
    }
}
