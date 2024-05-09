use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::{MajorDir, Numpad};
use crate::rules::Rule;

#[derive(Copy, Clone, Default, Debug)]
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

    let (init, iterate) = match &digits {
        Some(digits) => (digits.clone(), false),
        None => (Digits::zero(), true),
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
        let mut matches = vec![];
        let mut matches_a = vec![];
        let mut matches_b = vec![];
        let (a, b) = seq.split(3);
        for rule in &rules {
            match rule.matches(&seq) {
                Some(match_info) => matches.push(match_info),
                None => (),
            }
            match rule.matches(&a) {
                Some(_match_info) => matches_a.push(_match_info),

                None => (),
            }
            match rule.matches(&b) {
                Some(_match_info) => matches_b.push(_match_info),

                None => (),
            }
            if !matches.is_empty() || (!matches_a.is_empty() && !matches_b.is_empty()) {
                break;
            }
        }
        if !matches.is_empty() || (!matches_a.is_empty() && !matches_b.is_empty()) {
            matching += 1;

            if digits.is_some() {
                println!("Direct matches for {digits:?}:");
                for m in matches {
                    println!("{m:?}")
                }

                println!("Split matches a:");
                for m in matches_a {
                    println!("{m:?}")
                }

                println!("Split matches b:");
                for m in matches_b {
                    println!("{m:?}")
                }

                println!("--")
            }

            info.update(Info {
                total,
                matches: matching,
            });
        }
        seq.incr();
        if seq == init || !iterate {
            break;
        }
    }
    info.update(Info {
        total,
        matches: matching,
    });
    let ratio = matching as f64 / total as f64 * 100f64;
    println!("total: {} matching: {} {:.2}%", total, matching, ratio);
}
