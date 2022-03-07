use crate::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::Orientation::North;
use clap::ArgMatches;
use data_encoding::HEXLOWER_PERMISSIVE;
use hex_literal::hex;
use sha1::digest::Output;
use sha1::{Digest, Sha1};
use std::env;
use std::path::Path;
use svg::node::element::path::{Command, Data, Position};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

fn main() {
    let matches = clap::command!()
        .arg(clap::arg!([text] "Text to operate on").required(true))
        .arg(clap::arg!(-f --filename [filename] "Optional filename to save as").required(false))
        .get_matches();

    let (input, save_to) = Hex::names(&matches);

    println!("Value for text: '{}'", input);
    println!("Value for filename: {}", save_to);
    println!("sha: {}", Hex::sha1(input.clone()));

    Hex::parse(input);

    // create a Sha1 object
    let mut hasher = Sha1::new();

// process input message
    hasher.update(b"Rust in Action");

// acquire hash digest in the form of GenericArray,
// which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("fd79b29170b05c7454bdb796d7b61fde5d0f0990"));
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
pub struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    pub fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }
}

pub struct Hex;

impl Hex {
    pub fn names(matches: &ArgMatches) -> (String, String) {
        let input = String::from(matches.value_of("text").unwrap());
        let filename = matches.value_of("filename");

        let save_to = match filename {
            Some(name) => String::from(name),
            _ => {
                format!("{}.svg", input)
            }
        };
        (input, save_to)
    }

    pub fn parse(input: String) -> Vec<Operation> {
        let mut steps = Vec::<Operation>::new();
        for byte in input.bytes() {
            let step = match byte {
                b'0' => Home,
                b'1'..=b'9' => {
                    let distance = (byte - 0x30) as isize; // <12>
                    Forward(distance * (HEIGHT / 10))
                }
                b'a' | b'b' | b'c' => TurnLeft,
                b'd' | b'e' | b'f' => TurnRight,
                _ => Noop(byte), // <13>
            };
            println!("{:?}", step);
            steps.push(step);
        }
        steps
    }

    pub fn convert(operations: &Vec<Operation>) -> Vec<Command> {
        let mut turtle = Artist::new();

        let mut path_data = Vec::<Command>::with_capacity(operations.len());

        path_data
    }

    pub fn sha1(s: String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(s.as_bytes());
        HEXLOWER_PERMISSIVE.encode(hasher.finalize().as_ref())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod hex_tests {
    use super::*;

    #[test]
    fn sha1() {
        let name = "Rust in Action".to_string();
        let sha1 = Hex::sha1(name.clone());

        assert_eq!("5deaed72594aaa10edda990c5a5eed868ba8915e", sha1);
    }
}
