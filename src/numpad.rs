use crate::digit::Digit;
#[derive(Debug, Clone, Copy)]
pub struct XY(pub usize, pub usize);

impl XY {
    pub fn new() -> XY {
        XY(0, 0)
    }

    pub fn distance(self, other: XY) -> (isize, isize) {
        (
            other.0 as isize - self.0 as isize,
            other.1 as isize - self.1 as isize,
        )
    }
}

#[derive(Clone)]
pub struct Numpad {
    digits: Vec<Vec<Option<Digit>>>,
    dims: XY,
}

impl std::fmt::Debug for Numpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.digits {
            write!(f, "[")?;
            for digit_opt in row {
                match digit_opt {
                    Some(digit) => write!(f, " {:?},", digit.0)?,
                    None => write!(f, "  ,")?,
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MajorDir {
    XMajor,
    YMajor,
}

fn move_in(xy: XY, dims: &XY, major_dir: MajorDir) -> Option<XY> {
    match major_dir {
        MajorDir::XMajor => {
            let x = xy.0 + 1;
            if x == dims.0 {
                let (x, y) = (0, xy.1 + 1);
                if y == dims.1 {
                    None
                } else {
                    Some(XY(x, y))
                }
            } else {
                Some(XY(x, xy.1))
            }
        }
        MajorDir::YMajor => {
            let y = xy.1 + 1;
            if y == dims.1 {
                let (x, y) = (xy.0 + 1, 0);
                if x == dims.0 {
                    None
                } else {
                    Some(XY(x, y))
                }
            } else {
                Some(XY(xy.0, y))
            }
        }
    }
}

impl Numpad {
    pub fn new(major_dir: MajorDir, len: usize) -> Self {
        let (dims, dims19) = match major_dir {
            MajorDir::XMajor => {
                let width = len;
                let height = (10 + width - 1) / width;
                let height19 = (9 + width - 1) / width;
                let dims = XY(width, height);
                let dims19 = XY(width, height19);
                (dims, dims19)
            }
            MajorDir::YMajor => {
                let height = len;
                let width = (10 + height - 1) / height;
                let width19 = (9 + height - 1) / height;
                let dims = XY(width, height);
                let dims19 = XY(width19, height);
                (dims, dims19)
            }
        };
        let mut digits = vec![vec![None; dims.0]; dims.1];
        let mut xy = XY::new();
        let init_digit = Digit(1u8);
        let mut digit = init_digit;
        loop {
            digits[xy.1][xy.0] = Some(digit);
            digit.incr();
            if digit == init_digit {
                break;
            }
            //dbg!(xy);
            if let Some(next_xy) = move_in(xy, &dims19, major_dir) {
                xy = next_xy;
            } else {
                assert!(digit == Digit(0u8));
                // Count how many slots until None
                let mut slots_left = 0usize;
                let mut slot_xy = xy;
                while let Some(next_xy) = move_in(slot_xy, &dims, major_dir) {
                    slot_xy = next_xy;
                    slots_left += 1;
                }
                //dbg!(slots_left / 2);
                for _ in 0..=slots_left / 2 {
                    let Some(next_xy) = move_in(xy, &dims, major_dir) else {
                        todo!()
                    };
                    xy = next_xy;
                }
                //dbg!(xy);
            }
        }
        Numpad { digits, dims }
    }

    pub fn dims(&self) -> XY {
        self.dims
    }

    pub fn get(&self, xy: XY) -> Option<Digit> {
        self.digits[xy.1][xy.0]
    }

    pub fn xy_of_digit(&self, digit: Digit) -> XY {
        for y in 0..self.dims.1 {
            for x in 0..self.dims.0 {
                if self.digits[y][x] == Some(digit) {
                    return XY(x, y);
                }
            }
        }
        panic!("Digit not found from pad")
    }
}

