use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::{MajorDir, Numpad};
use crate::rules::Rule;

#[derive(Copy, Clone, Default,Debug)]
pub struct Info {
    pub matches: usize,
    pub total: usize,
}

pub trait InfoSignal {
    fn update(&self, info: Info);
}

pub fn parse_digits(s: &str) -> Vec<Digit> {
    let mut digits = vec![];
    for (_i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            digits.push(Digit(d as u8));
        } else {
            panic!("Invalid digit: {}", c);
        }
    }
    digits
}

pub fn find<INFO: InfoSignal>(digits: Option<Digits>, info: INFO) {
    let mut numpads = Vec::new();
    numpads.push(Numpad::new(MajorDir::XMajor, 1));
    numpads.push(Numpad::new(MajorDir::XMajor, 2));
    numpads.push(Numpad::new(MajorDir::XMajor, 3));
    numpads.push(Numpad::new(MajorDir::XMajor, 5));
    numpads.push(Numpad::new(MajorDir::XMajor, 10));
    numpads.push(Numpad::new(MajorDir::YMajor, 2));
    numpads.push(Numpad::new(MajorDir::YMajor, 3));
    numpads.push(Numpad::new(MajorDir::YMajor, 5));

    let (init, iterate) =
        match digits {
            Some(digits) => 
                (digits, false),
            None => (Digits::zero(), true)
        };

    let mut seq = init.clone();
    let mut total = 0usize;
    let mut matching = 0usize;
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    {
        use crate::rules::*;
        rules.push(Box::new(Incrementing));

        for numpad in &numpads {
            rules.push(Box::new(Worm::new(numpad)));
            rules.push(Box::new(DiagWorm::new(numpad)));
        }
        rules.push(Box::new(Worm::new(&numpads[2])));
    }
    loop {
        total += 1;
        let mut matches = false;
        let mut matches_a = false;
        let mut matches_b = false;
        let (a, b) = seq.split(3);
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
            if matches || (matches_a && matches_b) {
                break;
            }
        }
        if matches || (matches_a && matches_b) {
            matching += 1;
            info.update(Info { total, matches: matching });
        }
        seq.incr();
        if seq == init || !iterate {
            break;
        }
    }
    info.update(Info { total, matches: matching });
    let ratio = matching as f64 / total as f64 * 100f64;
    println!("total: {} matching: {} {:.2}%", total, matching, ratio);
}
