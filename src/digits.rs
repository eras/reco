use crate::digit::Digit;

const NUM_DIGITS: usize = 6;

#[derive(Clone, PartialEq)]
pub struct Digits {
    pub digits: Vec<Digit>,
    pub str: String, // cached: created/updated during init/mutation
}

const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl std::fmt::Debug for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.str)
    }
}

impl Digits {
    pub fn zero() -> Self {
        let digits = [Digit(0u8); NUM_DIGITS];
        Digits {
            digits: digits.to_vec(),
            str: Self::str_of_digits(&digits),
        }
    }

    pub fn str_of_digits(digits: &[Digit]) -> String {
        let mut str = String::new();
        for c in 0..digits.len() {
            str.push(DIGIT_CHARS[digits[c].0 as usize])
        }
        str
    }

    pub fn incr(&mut self) {
        let mut digit = self.digits.len();
        while digit > 0 {
            self.digits[digit - 1].incr();
            if self.digits[digit - 1].0 != 0 {
                break;
            }
            digit -= 1
        }
        self.str = Self::str_of_digits(&self.digits[..]);
    }

    pub fn digits(&self) -> &[Digit] {
        &self.digits[..]
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    pub fn split_in_half(&self) -> (Digits, Digits) {
        (
            Digits::from(&self.digits[0..self.digits.len() / 2]),
            Digits::from(&self.digits[self.digits.len() / 2..self.digits.len()]),
        )
    }
}

impl From<&[Digit]> for Digits {
    fn from(digits: &[Digit]) -> Self {
        let str = Self::str_of_digits(&digits[..]);
        Self {
            digits: digits.to_vec(),
            str,
        }
    }
}

