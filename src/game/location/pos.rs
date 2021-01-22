use euclid::{point2, vec2, Box2D, Point2D, Rect, Vector2D};

struct Unit {}

pub trait Positionable {
    fn position(&self) -> Position;
}

#[derive(Clone, Debug, Copy)]
pub struct Position {
    internal: Point2D<f64, Unit>,
}

impl Position {
    pub fn new() -> Self {
        Position {
            internal: point2(0 as f64, 0 as f64),
        }
    }

    pub fn from_coord(x: f64, y: f64) -> Self {
        Position {
            internal: point2(x, y),
        }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.internal.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.internal.y
    }
}

#[derive(Debug)]
pub struct Area {
    internal: Box2D<f64, Unit>,
}

impl Area {
    pub fn new() -> Self {
        Area {
            internal: Box2D::new(
                Point2D::new(0 as f64, 0 as f64),
                Point2D::new(0 as f64, 0 as f64),
            ),
        }
    }

    pub fn from_point(min: (f64, f64), max: (f64, f64)) -> Self {
        Area {
            internal: Box2D::new(Point2D::new(min.0, min.1), Point2D::new(max.0, max.1)),
        }
    }

    pub fn min(&self) -> Position {
        Position::from_coord(self.internal.min.x, self.internal.min.y)
    }

    pub fn max(&self) -> Position {
        Position::from_coord(self.internal.max.x, self.internal.max.y)
    }

    pub fn contains(&self, pos: Position) -> bool {
        self.internal.contains(pos.internal)
    }
}
