use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::Numpad;

pub struct MatchInfo {}

use regex;

pub trait Rule {
    fn matches(&self, seq: &Digits) -> Option<MatchInfo>;
}

pub struct Incrementing;

impl Rule for Incrementing {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut prev_digit = digits.digits()[0];
        let diff = digits.digits()[1].sub(digits.digits()[0]);
        for c in 1..digits.len() {
            let cur_digit = digits.digits()[c];
            let cur_diff = cur_digit.sub(prev_digit);
            if cur_diff != diff {
                return None;
            }
            prev_digit = cur_digit
        }
        Some(MatchInfo {})
    }
}

pub struct Reverse(Box<dyn Rule>);

impl Rule for Reverse {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut rev_seq = vec![Digit(0u8); digits.len()];
        for c in 0..digits.len() {
            rev_seq[c] = digits.digits()[digits.len() - c - 1];
        }
        self.0.matches(&Digits::from(&rev_seq[..]))
    }
}

pub struct Regex(regex::Regex);

impl Regex {
    pub fn new(re: &str) -> Self {
        Regex(regex::Regex::new(re).unwrap())
    }
}

impl Rule for Regex {
    fn matches(&self, seq: &Digits) -> Option<MatchInfo> {
        if self.0.is_match(&seq.str) {
            Some(MatchInfo {})
        } else {
            None
        }
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
                && ((cur_xy.0 == 0 && xy.0 == x_wrap_size)
                    || (cur_xy.0 == x_wrap_size && xy.0 == 0))
            {
                ()
            } else if cur_distance.0 == 0
                && ((cur_xy.1 == 0 && xy.1 == y_wrap_size)
                    || (cur_xy.1 == x_wrap_size && xy.1 == 0))
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
        Some(MatchInfo {})
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
        Some(MatchInfo {})
    }
}

