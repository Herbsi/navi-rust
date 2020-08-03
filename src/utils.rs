use std::fmt;

///////////////////////////////////////////////////////////////////////////////
//                                   Angle                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum Angle {
    Straight,
    Turn(f64, Direction), // in degrees
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

impl From<bool> for Direction {
    fn from(from: bool) -> Direction {
        match from {
            false => Direction::Left,
            true => Direction::Right,
        }
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Angle::Straight => write!(f, "Straight"),
            Angle::Turn(angle, Direction::Left) => write!(f, "Left by {:<5.2}", angle),
            Angle::Turn(angle, Direction::Right) => write!(f, "Right by {:<5.2}", angle),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                    Edge                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Edge {
    pub node: usize,
    pub cost: f64,
}

///////////////////////////////////////////////////////////////////////////////
//                                    Prev                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum Prev {
    Start,
    Undefined,
    Node(usize),
}
