use core::fmt;

#[derive(Debug, Default)]
enum Transform {
    #[default]
    None,
    Decrease(usize),
    Increase(usize)
}

impl Transform {
    fn from(v: i32) -> Transform {
        if v.is_positive() {
            Transform::Increase(v as usize)
        } else if v.is_negative() {
            Transform::Decrease((v * -1) as usize)
        } else {
            Transform::None
        }
    }

    fn apply(&self, value: usize) -> Option<usize> {
        return match self {
            Transform::None => Some(value),
            Transform::Decrease(amount) => {
                let (_, overflow) = value.overflowing_sub(*amount);
                if overflow { None }
                else { Some(value - amount) }
            },
            Transform::Increase(amount) =>{
                let (_, overflow) = value.overflowing_add(*amount);
                if overflow { None }
                else { Some(value + amount) }
            },
        }
    }

    fn reverse(&self) -> Transform {
        match self {
            Transform::None => Transform::None,
            Transform::Decrease(v) => Transform::Increase(*v),
            Transform::Increase(v) => Transform::Decrease(*v),
        }
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::None => write!(f, "0"),
            Transform::Decrease(v) => write!(f, "-{v}"),
            Transform::Increase(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn offset_from(&self, other: &Point) -> Offset {
        let (_, x_overflow) = self.x.overflowing_sub(other.x);
        let x = if x_overflow { Transform::Decrease(self.x.abs_diff(other.x)) } else { Transform::Increase(self.x.abs_diff(other.x)) };

        let (_, y_overflow) = self.y.overflowing_sub(other.y);
        let y = if y_overflow { Transform::Decrease(self.y.abs_diff(other.y)) } else { Transform::Increase(self.y.abs_diff(other.y)) };

        Offset { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Rect {
    pub width: usize,
    pub height: usize
}

impl Rect {
    pub fn contains(&self, point: &Point) -> bool {
        point.x < self.width && point.y < self.height
    }
}

#[derive(Debug, Default)]
pub struct Offset {
    x: Transform,
    y: Transform
}

impl Offset {
    pub fn from(x: i32, y: i32) -> Offset {
        Offset {
            x: Transform::from(x),
            y: Transform::from(y)
        }
    }

    pub fn apply(&self, point: &Point) -> Option<Point> {
        let x = self.x.apply(point.x)?;
        let y = self.y.apply(point.y)?;

        return Some(Point { x, y });
    }

    // TODO Move this to another abstraction representing the Plane, get point from Plane
    pub fn apply_within(&self, point: &Point, limit: &Point) -> Option<Point> {
        let x = self.x.apply(point.x)?;
        let y = self.y.apply(point.y)?;
        if x > limit.x || y > limit.y {
            return None;
        } else {
            return Some(Point { x, y })
        }
    }

    pub fn inverse(&self) -> Offset {
        return Offset { x: self.x.reverse(), y: self.y.reverse() };
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn offset(&self) -> Offset {
        match self {
            Direction::Up => Offset { y: Transform::Decrease(1), ..Default::default() },
            Direction::Right => Offset { x: Transform::Increase(1), ..Default::default() },
            Direction::Down => Offset { y: Transform::Increase(1), ..Default::default() },
            Direction::Left => Offset { x: Transform::Decrease(1), ..Default::default() },
        }
    }

    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
