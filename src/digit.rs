#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Digit(pub u8);

impl Digit {
    pub fn incr(&mut self) {
        if self.0 == 9 {
            self.0 = 0
        } else {
            self.0 += 1
        }
    }

    pub fn succ(self) -> Self {
        let mut copy = self.clone();
        copy.incr();
        copy
    }

    pub fn add(self, other: i8) -> Self {
        Digit((self.0 as i8 + other).rem_euclid(10) as u8)
    }

    pub fn sub(self, other: Digit) -> Self {
        Digit((self.0 as i8 - other.0 as i8).rem_euclid(10) as u8)
    }
}

