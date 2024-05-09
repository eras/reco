use clap::{App, Arg};
use regex;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Digit(u8);

const NUM_DIGITS: usize = 6;

impl Digit {
    fn incr(&mut self) {
        if self.0 == 9 {
            self.0 = 0
        } else {
            self.0 += 1
        }
    }

    fn succ(self) -> Self {
        let mut copy = self.clone();
        copy.incr();
        copy
    }

    fn add(self, other: i8) -> Self {
        Digit((self.0 as i8 + other).rem_euclid(10) as u8)
    }

    fn sub(self, other: Digit) -> Self {
        Digit((self.0 as i8 - other.0 as i8).rem_euclid(10) as u8)
    }
}

#[derive(Clone, PartialEq)]
struct Digits {
    digits: Vec<Digit>,
    str: String, // cached: created/updated during init/mutation
}

const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl std::fmt::Debug for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.str)
    }
}

impl Digits {
    fn zero() -> Self {
        let digits = [Digit(0u8); NUM_DIGITS];
        Digits {
            digits: digits.to_vec(),
            str: Self::str_of_digits(&digits),
        }
    }

    fn str_of_digits(digits: &[Digit]) -> String {
        let mut str = String::new();
        for c in 0..digits.len() {
            str.push(DIGIT_CHARS[digits[c].0 as usize])
        }
        str
    }

    fn incr(&mut self) {
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

    fn digits(&self) -> &[Digit] {
        &self.digits[..]
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn split_in_half(&self) -> (Digits, Digits) {
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

struct MatchInfo {}

trait Rule {
    fn matches(&self, seq: &Digits) -> Option<MatchInfo>;
}

struct Incrementing;

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

struct Reverse(Box<dyn Rule>);

impl Rule for Reverse {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut rev_seq = vec![Digit(0u8); digits.len()];
        for c in 0..digits.len() {
            rev_seq[c] = digits.digits()[digits.len() - c - 1];
        }
        self.0.matches(&Digits::from(&rev_seq[..]))
    }
}

struct Regex(regex::Regex);

impl Regex {
    fn new(re: &str) -> Self {
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

#[derive(Debug, Clone, Copy)]
struct XY(usize, usize);

impl XY {
    fn new() -> XY {
        XY(0, 0)
    }

    fn distance(self, other: XY) -> (isize, isize) {
        (
            other.0 as isize - self.0 as isize,
            other.1 as isize - self.1 as isize,
        )
    }
}

#[derive(Clone)]
struct Numpad {
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
enum MajorDir {
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
    fn new(major_dir: MajorDir, len: usize) -> Self {
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

    fn dims(&self) -> XY {
        self.dims
    }

    fn get(&self, xy: XY) -> Option<Digit> {
        self.digits[xy.1][xy.0]
    }

    fn xy_of_digit(&self, digit: Digit) -> XY {
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

struct Worm<'a> {
    numpad: &'a Numpad,
}

impl<'a> Worm<'a> {
    fn new(numpad: &'a Numpad) -> Self {
        Worm { numpad }
    }
}

impl<'a> Rule for Worm<'a> {
    fn matches(&self, digits: &Digits) -> Option<MatchInfo> {
        let mut xy = self.numpad.xy_of_digit(digits.digits()[0]);
        let x_wrap_size = self.numpad.dims.0 - 1;
        let y_wrap_size = self.numpad.dims.1 - 1;
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

struct DiagWorm<'a> {
    numpad: &'a Numpad,
}

impl<'a> DiagWorm<'a> {
    fn new(numpad: &'a Numpad) -> Self {
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

fn main() {
    let matches = App::new("Remember Code")
        .arg(
            Arg::with_name("digits")
                .takes_value(true)
                .index(1)
                .help("Digits to query"),
        )
        .get_matches();

    let digits = matches.value_of("digits").unwrap_or("");

    let mut numpads = Vec::new();
    numpads.push(Numpad::new(MajorDir::XMajor, 1));
    numpads.push(Numpad::new(MajorDir::XMajor, 2));
    numpads.push(Numpad::new(MajorDir::XMajor, 3));
    numpads.push(Numpad::new(MajorDir::XMajor, 5));
    numpads.push(Numpad::new(MajorDir::XMajor, 10));
    numpads.push(Numpad::new(MajorDir::YMajor, 2));
    numpads.push(Numpad::new(MajorDir::YMajor, 3));
    numpads.push(Numpad::new(MajorDir::YMajor, 5));
    let numpads = numpads;

    let (init, iterate) = if !digits.is_empty() {
        (Digits::from(&parse_digits(digits)[..]), false)
    } else {
        (Digits::zero(), true)
    };

    let mut seq = init.clone();
    let mut total = 0usize;
    let mut matching = 0usize;
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    rules.push(Box::new(Incrementing));
    //rules.push(Box::new(Reverse(Box::new(Incrementing))));
    //rules.push(Box::new(Regex::new(r"^024680")));

    for numpad in &numpads {
        rules.push(Box::new(Worm::new(numpad)));
        rules.push(Box::new(DiagWorm::new(numpad)));
    }
    rules.push(Box::new(Worm::new(&numpads[2])));
    loop {
        total += 1;
        let mut matches = false;
        let mut matches_a = false;
        let mut matches_b = false;
        let (a, b) = seq.split_in_half();
        for rule in &rules {
            match rule.matches(&seq) {
                Some(_match_info) => {
                    matches = true;
                }
                None => (),
            }
            match rule.matches(&a) {
                Some(_match_info) => {
                    matches_a = true;
                }
                None => (),
            }
            match rule.matches(&b) {
                Some(_match_info) => {
                    matches_b = true;
                }
                None => (),
            }
        }
        if matches || (matches_a && matches_b) {
            //println!("{seq:?}");
            matching += 1;
        } else {
            //println!("{seq:?}");
        }
        seq.incr();
        if seq == init || !iterate {
            break;
        }
    }
    let ratio = matching as f64 / total as f64 * 100f64;
    println!("total: {} matching: {} {:.2}%", total, matching, ratio);
}

fn parse_digits(s: &str) -> Vec<Digit> {
    let mut digits = vec![];
    for (i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            digits.push(Digit(d as u8));
        } else {
            panic!("Invalid digit: {}", c);
        }
    }
    digits
}
