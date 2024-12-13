use math::IVec2;

pub mod map;
pub mod math;

//pub const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub const CARDINAL_DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: -1 },
];

pub const ENUM_EAST_WEST_DIRS: [CardinalDirection; 2] =
    [CardinalDirection::West, CardinalDirection::East];
pub const ENUM_NORTH_SOUTH_DIRS: [CardinalDirection; 2] =
    [CardinalDirection::North, CardinalDirection::South];
pub const ENUM_CARDINAL_DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::East,
    CardinalDirection::South,
    CardinalDirection::West,
];

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum CardinalDirection {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl CardinalDirection {
    pub fn get_direction(&self) -> IVec2 {
        CARDINAL_DIRECTIONS[*self as usize]
    }
}

#[inline]
pub fn cardinal_rot_right_90(direction_index: usize) -> usize {
    (direction_index + 1) % CARDINAL_DIRECTIONS.len()
}

pub const ALPHABET_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
