use super::math::{IVec2, UVec2};

#[derive(Clone)]
pub struct CharMap {
    pub data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl CharMap {
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = data.len();
        let height = data[0].len();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, vec: UVec2) -> char {
        self.data[vec.x][vec.y]
    }

    pub fn get_ivec_checked(&self, vec: IVec2) -> Option<char> {
        if vec.is_coord_safe(self.width, self.height) {
            Some(self.data[vec.x as usize][vec.y as usize])
        } else {
            None
        }
    }

    pub fn get_ivec_unchecked(&self, vec: IVec2) -> char {
        self.data[vec.x as usize][vec.y as usize]
    }

    pub fn set(&mut self, vec: UVec2, c: char) {
        self.data[vec.x][vec.y] = c;
    }

    pub fn swap(&mut self, pos1: UVec2, pos2: UVec2) {
        let a = self.get(pos1);
        let b = self.get(pos2);
        self.set(pos1, b);
        self.set(pos2, a);
    }
}

pub struct IntMap {
    pub data: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl IntMap {
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = data.len();
        let height = data[0].len();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, vec: UVec2) -> u8 {
        self.data[vec.x][vec.y]
    }

    pub fn get_ivec_checked(&self, vec: IVec2) -> Option<u8> {
        if vec.is_coord_safe(self.width, self.height) {
            Some(self.data[vec.x as usize][vec.y as usize])
        } else {
            None
        }
    }

    pub fn get_ivec_unchecked(&self, vec: IVec2) -> u8 {
        self.data[vec.x as usize][vec.y as usize]
    }
}
