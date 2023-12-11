use crate::number::Number;

#[derive(Default, Debug, Clone)]
pub struct Symbol {
    pub pos: (u32, u32),
    pub symbol: char,
}

#[derive(Default, Debug)]
pub struct Gear {
    pub symbol: Symbol,
    pub gears: Vec<Number>,
}

impl Gear {
    pub fn ratio(&self) -> u32 {
        if self.gears.len() == 1 {
            0
        } else {
            self.gears[0].value * self.gears[1].value
        }
    }
}
