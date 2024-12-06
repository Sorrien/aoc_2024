use std::ops::Add;

pub const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

pub fn solution(input: String) -> u32 {
    let map = input
        .lines()
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map.len();
    let height = map[0].len();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .filter_map(|(x, line)| {
            if let Some(y) = line.iter().position(|c| *c == '^') {
                Some((x, y))
            } else {
                None
            }
        })
        .next()
        .unwrap();

    let mut current_pos = UVec2::new(start_x, start_y);

    let mut is_in_bounds = true;
    let mut dir = Direction::North;
    let mut visited_positions = vec![current_pos];

    while is_in_bounds {
        let dir_vec = dir.get_x_y();
        let next_position = current_pos + dir_vec;

        if next_position.x < 0
            || next_position.x >= width as isize
            || next_position.y < 0
            || next_position.y >= height as isize
        {
            is_in_bounds = false;
        } else {
            let next_char = map[next_position.x as usize][next_position.y as usize];
            if next_char == '#' {
                dir = dir.rot_right_90();
            } else {
                current_pos = next_position.as_u_position();
                visited_positions.push(current_pos);
            }
        }
    }

    visited_positions.sort();
    visited_positions.dedup();

    visited_positions.len() as u32
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl Direction {
    pub fn get_x_y(&self) -> IVec2 {
        let (x, y) = DIRECTIONS[*self as usize];
        IVec2::new(x, y)
    }
    pub fn rot_right_90(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl UVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_i_position(&self) -> IVec2 {
        IVec2::new(self.x as isize, self.y as isize)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl IVec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn as_u_position(&self) -> UVec2 {
        UVec2::new(self.x as usize, self.y as usize)
    }
}

impl Add<IVec2> for UVec2 {
    fn add(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x as isize + rhs.x, self.y as isize + rhs.y)
    }

    type Output = IVec2;
}

impl Add<UVec2> for IVec2 {
    fn add(self, rhs: UVec2) -> IVec2 {
        IVec2::new(self.x + rhs.x as isize, self.y + rhs.y as isize)
    }

    type Output = IVec2;
}
