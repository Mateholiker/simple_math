use super::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum LineType {
    ///Gerade
    Line,
    ///Halbgrade
    Ray,
    ///Strecke
    LineSegment,
}
#[derive(Clone, Copy, Debug)]
pub struct Line2 {
    start: Vec2,
    end: Vec2,
    line_type: LineType,
}

impl Line2 {
    pub fn from_support_point_and_vector(
        support_point: Vec2,
        vector: Vec2,
        line_type: LineType,
    ) -> Line2 {
        Line2::from_start_and_end(support_point, support_point + vector, line_type)
    }

    pub fn from_start_and_end(start: Vec2, end: Vec2, line_type: LineType) -> Line2 {
        Line2 {
            start,
            end,
            line_type,
        }
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    pub fn line_type(&self) -> LineType {
        self.line_type
    }
}

impl Line2 {
    pub fn intersection(&self, other: Line2) -> Option<Vec2> {
        //the following algorithm is from https://stackoverflow.com/questions/385305/efficient-maths-algorithm-to-calculate-intersections
        let s1 = self.start;
        let e1 = self.end;
        let s2 = other.start;
        let e2 = other.end;
        //delta Line self
        let d1 = e1 - s1;
        //delta Line other
        let d2 = e2 - s2;

        let determinante = d1.x() * d2.y() - d1.y() * d2.x();
        if determinante.abs() > 0.001 {
            //delta Start Points
            let ds = s1 - s2;

            //intersection pos for self
            let p1 = (ds.y() * d2.x() - ds.x() * d2.y()) / determinante;

            //intersection pos for other
            let p2 = (ds.y() * d1.x() - ds.x() * d1.y()) / determinante;

            if ((s2 + d2 * p2) - (s1 + d1 * p1)).euclidean_lenght() >= 0.1 {
                dbg!(((s2 + d2 * p2) - (s1 + d1 * p1)).euclidean_lenght());
                assert!(((s2 + d2 * p2) - (s1 + d1 * p1)).euclidean_lenght() < 0.1);
            }

            use LineType::{Line, LineSegment, Ray};
            let intersection_on_self = match self.line_type {
                Line => true,
                Ray => p1 >= 0.0,
                LineSegment => (0.0..=1.0).contains(&p1),
            };

            let intersection_on_other = match other.line_type {
                Line => true,
                Ray => p2 >= 0.0,
                LineSegment => (0.0..=1.0).contains(&p2),
            };

            if intersection_on_self && intersection_on_other {
                Some(s1 + d1 * p1)
            } else {
                None
            }
        } else {
            //lines are parallel
            None
        }
    }
}
