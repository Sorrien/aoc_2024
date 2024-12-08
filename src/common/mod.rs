use math::IVec2;

pub mod math;

pub const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
pub const CARDINAL_INTERCARDINAL_DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];
pub const INTERCARDINAL_DIRECTIONS: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

#[derive(Clone, Copy, Debug)]
pub enum CardinalDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl CardinalDirection {
    pub fn get_x_y(&self) -> IVec2 {
        let (x, y) = CARDINAL_DIRECTIONS[*self as usize];
        IVec2::new(x, y)
    }
    pub fn rot_right_90(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::West => CardinalDirection::North,
        }
    }
}
