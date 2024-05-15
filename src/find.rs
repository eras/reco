use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::{MajorDir, Numpad};
use crate::rules::Rule;

#[derive(Clone, Default, Debug)]
pub struct Info {
    pub matches: usize,
    pub total: usize,
    pub message: String,
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
        rules.push(Box::new(Arithmetic));

        for numpad in &numpads {
            rules.push(Box::new(Worm::new(numpad)));
            rules.push(Box::new(DiagWorm::new(numpad)));
        }
        rules.push(Box::new(Worm::new(&numpads[2])));
    }
    let mut message = String::from("");
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

            message = String::from("");

            if digits.is_some() {
                message += &format!("Direct matches for {digits:?}:\n");
                for m in matches {
                    message += &format!("{m:?}\n")
                }

                message += &format!("Split matches a:\n");
                for m in matches_a {
                    message += &format!("{m:?}\n")
                }

                message += &format!("Split matches b:\n");
                for m in matches_b {
                    message += &format!("{m:?}\n")
                }

                message += &format!("--\n")
            }

            info.update(Info {
                total,
                matches: matching,
                message: message.clone(),
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
        message: message,
    });
    let ratio = matching as f64 / total as f64 * 100f64;
    println!("total: {} matching: {} {:.2}%", total, matching, ratio);
}
