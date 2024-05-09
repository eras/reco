use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::Numpad;

pub struct MatchInfo {
    message: String,
}

impl std::fmt::Debug for MatchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        Ok(())
    }
}

use regex;

pub trait Rule {
    fn matches(&self, seq: &Digits) -> Option<MatchInfo>;

    fn name(&self) -> &str;
}



pub struct Incrementing;

impl Rule for Incrementing {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let digits = digits.digits();
        let mut prev_digit = digits[0];
        let diff = digits[1].sub(digits[0]);
        for c in 1..digits.len() {
            let cur_digit = digits[c];
            let cur_diff = cur_digit.sub(prev_digit);
            if cur_diff != diff {
                return None;
            }
            prev_digit = cur_digit
        }
        Some(MatchInfo { message: format!("Incrementing") })
    }

    fn name(&self) -> &str {
        "Incrementing"
    }
}

pub struct Reverse{
    rule: Box<dyn Rule>,
    name: String,
}

impl Reverse {
    pub fn new(rule: Box<dyn Rule>) -> Self {
        let name = format!("Reverse of {}", rule.name());
        Reverse {
            rule,
            name,
        }
    }
}

impl Rule for Reverse {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut rev_seq = vec![Digit(0u8); digits.len()];
        for c in 0..digits.len() {
            rev_seq[c] = digits.digits()[digits.len() - c - 1];
        }
        self.rule.matches(&Digits::from(&rev_seq[..]))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Regex{
    re: regex::Regex,
    name: String,
}

impl Regex {
    pub fn new(re: &str) -> Self {
        let re = regex::Regex::new(re).unwrap();
        let name = format!("Regex {re}");
        Regex{ re, name }
    }
}

impl Rule for Regex {
    fn matches(&self, seq: &Digits) -> Option<MatchInfo> {
        if self.re.is_match(&seq.str) {
            Some(MatchInfo { message: format!("Regex") })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Worm<'a> {
    numpad: &'a Numpad,
}

impl<'a> Worm<'a> {
    pub fn new(numpad: &'a Numpad) -> Self {
        Worm { numpad }
    }
}

impl<'a> Rule for Worm<'a> {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut xy = self.numpad.xy_of_digit(digits.digits()[0]);
        let x_wrap_size = self.numpad.dims().0 - 1;
        let y_wrap_size = self.numpad.dims().1 - 1;
        for c in 1..digits.len() {
            let cur_digit = digits.digits()[c];
            let cur_xy = self.numpad.xy_of_digit(cur_digit);
            let cur_distance = cur_xy.distance(xy);
            if cur_distance.1 == 0
                && (((cur_xy.0, xy.0) == (0, x_wrap_size))
                    || ((cur_xy.0, xy.0) == (x_wrap_size, 0)))
            {
                ()
            } else if cur_distance.0 == 0
                && (((cur_xy.1, xy.1) == (0, y_wrap_size))
                    || ((cur_xy.1, xy.1) == (x_wrap_size, 0)))
            {
                ()
            } else {
                match cur_distance {
                    (0, 0) | (0, 1) | (1, 0) | (-1, 0) | (0, -1) => (),
                    _ => return None,
                }
            }
            xy = cur_xy;
        }
        Some(MatchInfo { message: format!("Worm for\n{0:?}", self.numpad) })
    }

    fn name(&self) -> &str {
        "Worm"
    }
}

pub struct DiagWorm<'a> {
    numpad: &'a Numpad,
}

impl<'a> DiagWorm<'a> {
    pub fn new(numpad: &'a Numpad) -> Self {
        DiagWorm { numpad }
    }
}

impl<'a> Rule for DiagWorm<'a> {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut xy = self.numpad.xy_of_digit(digits.digits()[0]);
        for c in 1..digits.len() {
            let cur_digit = digits.digits()[c];
            let cur_xy = self.numpad.xy_of_digit(cur_digit);
            let cur_distance = cur_xy.distance(xy);
            match cur_distance {
                (0, 0) | (0, 2) | (2, 0) | (-2, 0) | (0, -2) => (),
                _ => return None,
            }
            xy = cur_xy;
        }
        Some(MatchInfo { message: format!("DiagWorm for\n{0:?}", self.numpad) })
    }

    fn name(&self) -> &str {
        "DiagWorm"
    }
}
