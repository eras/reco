mod digit;
mod digits;
mod rules;
mod numpad;

use clap::{App, Arg};

use crate::digit::Digit;
use crate::digits::Digits;
use crate::numpad::{Numpad, MajorDir};
use crate::rules::Rule;

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
    {
        use crate::rules::*;
        rules.push(Box::new(Incrementing));
        //rules.push(Box::new(Reverse(Box::new(Incrementing))));
        //rules.push(Box::new(Regex::new(r"^024680")));

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
