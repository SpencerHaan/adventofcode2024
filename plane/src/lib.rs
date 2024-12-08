#[derive(Debug)]
enum Transform {
    None,
    Decrease(usize),
    Increase(usize)
}

impl Transform {
    fn from(v: i32) -> Transform {
        if v > 0 {
            Transform::Increase(v as usize)
        } else if v < 0 {
            Transform::Decrease((v * -1) as usize)
        } else {
            Transform::None
        }
    }

    fn apply(&self, value: usize) -> Option<usize> {
        return match self {
            Transform::None => Some(value),
            Transform::Decrease(amount) => {
                if value == 0 { None }
                else { Some(value - amount) }
            },
            Transform::Increase(amount) => Some(value + amount),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
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

    // Move this to another abstraction representing the Plane, get point from Plane
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
            Direction::Up => Offset { x: Transform::None, y: Transform::Decrease(1) },
            Direction::Right => Offset { x: Transform::Increase(1), y: Transform::None },
            Direction::Down => Offset { x: Transform::None, y: Transform::Increase(1) },
            Direction::Left => Offset { x: Transform::Decrease(1), y: Transform::None },
        }
    }

    pub fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
