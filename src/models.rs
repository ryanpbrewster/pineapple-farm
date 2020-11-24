pub struct Grid {
    width: i32,
    elements: Vec<Cell>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid {
        Grid {
            width,
            elements: (0..width * height).map(|_| Cell::default()).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Idx, &Cell)> {
        let width = self.width;
        self.elements
            .iter()
            .enumerate()
            .map(move |(idx, value)| ((idx as i32 / width, idx as i32 % width), value))
    }

    pub fn get(&self, &(i, j): &Idx) -> Option<&Cell> {
        if i < 0 || j < 0 || j >= self.width {
            None
        } else {
            self.elements.get((i * self.width + j) as usize)
        }
    }
    pub fn get_mut(&mut self, &(i, j): &Idx) -> Option<&mut Cell> {
        if i < 0 || j < 0 || j >= self.width {
            None
        } else {
            self.elements.get_mut((i * self.width + j) as usize)
        }
    }

    pub fn get_heat(&self, &(i, j): &Idx) -> u32 {
        let mut heat = 0;
        for ii in i - 1..=i + 1 {
            for jj in j - 1..=j + 1 {
                if let Some(Cell { radiators, .. }) = self.get(&(ii, jj)) {
                    if *radiators > 0 {
                        seed::log(format!("{}/{} --{}--> {}/{}", ii, jj, radiators, i, j));
                    }
                    heat += match i32::abs(i - ii) + i32::abs(j - jj) {
                        0 => radiators * H0,
                        1 => radiators * H1,
                        2 => radiators * H2,
                        _ => 0,
                    }
                }
            }
        }
        heat
    }

    pub fn get_status(&self, idx: &Idx) -> Option<GrowthStatus> {
        self.get(idx).map(|cell| {
            let heat = self.get_heat(idx);
            if heat < MIN_HEAT {
                GrowthStatus::TooCold
            } else if heat <= MAX_HEAT {
                GrowthStatus::Fruiting(cell.capacity)
            } else if heat < OVERHEAT {
                GrowthStatus::TooHot
            } else {
                GrowthStatus::Overheated
            }
        })
    }

    pub fn total_growth(&self) -> u32 {
        self.iter()
            .map(|(idx, _)| match self.get_status(&idx) {
                Some(GrowthStatus::Fruiting(count)) => count,
                _ => 0,
            })
            .sum()
    }
}

type Idx = (i32, i32);

pub const MIN_HEAT: u32 = 3;
pub const MAX_HEAT: u32 = 6;
pub const OVERHEAT: u32 = 8;

pub const H0: u32 = 5;
pub const H1: u32 = 3;
pub const H2: u32 = 1;

#[derive(Debug)]
pub struct Cell {
    pub capacity: u32,
    pub radiators: u32,
}
impl Default for Cell {
    fn default() -> Cell {
        Cell {
            capacity: 5,
            radiators: 0,
        }
    }
}

pub enum GrowthStatus {
    TooCold,
    Fruiting(u32),
    TooHot,
    Overheated,
}
