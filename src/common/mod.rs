pub mod map;
pub mod math;

pub const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
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

#[inline]
pub fn cardinal_rot_right_90(direction_index: usize) -> usize {
    (direction_index + 1) % CARDINAL_DIRECTIONS.len()
}
