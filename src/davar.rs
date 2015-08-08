extern crate num;
extern crate rustc_serialize;

use std::vec::Vec;

pub mod simulate;

pub mod in_out;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, RustcDecodable, RustcEncodable)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
  fn new(x: i32, y:i32) -> Cell {
    Cell{x: x, y: y}
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, RustcDecodable, RustcEncodable)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction { W, E, SW, SE }

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Clock { Wise, Counter }

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Command {
    Move(Direction),
    Rotate(Clock),
}

impl Command {
    pub fn to_char(self) -> char {
        use Command::*;
        use Direction::*;
        match self {
            Move(W) => 'p',
            Move(E) => 'b',
            Move(SW) => 'a',
            Move(SE) => 'l',
            Rotate(Clock::Wise) => 'd',
            Rotate(Clock::Counter) => 'k',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Input {
    pub id: i32,
    pub units: Vec<Unit>,
    pub width: i32,
    pub height: i32,
    pub filled: Vec<Cell>,
    pub sourceLength: i32,
    pub sourceSeeds: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Solution {
    pub problemId: i32,
    pub seed: i32,
    pub tag: Option<String>,
    pub solution: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub filled_array: Vec<bool>,
    pub visited: Vec<Unit>,
    pub unit_sequence: Vec<Unit>, // holds the actual sequence of units
    pub ls_old: i32,
    pub score: i32,
    pub game_over: bool,
}

impl State {
    fn new() -> State {
        State {
            width: 10,
            height: 10,
            filled_array: vec![false; 10*10],
            visited: Vec::new(),
            unit_sequence: Vec::new(),
            ls_old: 0,
            score: 0,
            game_over: false,
        }
    }

    fn with_size(width: i32, height: i32) -> State {
        State {
            width: width,
            height: height,
            filled_array: vec![false; (width*height) as usize],
            visited: Vec::with_capacity(3*width as usize),
            unit_sequence: Vec::new(),
            ls_old: 0,
            score: 0,
            game_over: false,
        }
    }
    fn filled(&mut self, c: Cell) -> &mut bool {
        &mut self.filled_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }
    fn is_filled(&self, c: Cell) -> bool {
        if c.x < 0 || c.x >= self.width || c.y < 0 || c.y >= self.height {
            return false;
        }
        self.filled_array[c.x as usize + (c.y as usize)*(self.width as usize)]
    }

    fn visualize(&self) -> String {

        // print stuff here, but eventually return a string.
        let mut chars: Vec<Vec<char>> = vec![vec![]; self.width as usize];
        for i in 0 .. self.width as usize {
            chars[i] = vec!['O'; self.height as usize];
        }

        let mut out_str = Vec::with_capacity( (2*self.width + 4) as usize * self.height as usize);
        if self.unit_sequence.len() > 0 {
            let ref unit_array = self.unit_sequence[0].members;
            let ref pivot = self.unit_sequence[0].pivot;

            for y in 0..self.height as usize {
              out_str.push('|' as u8);
              if y%2==1 { out_str.push(' ' as u8); }

              for x in 0..self.width as usize {
                out_str.push(' ' as u8);

                let here = Cell::new(x as i32, y as i32);
                if self.is_filled(here) {    // Is cell filled?
                    out_str.push('X' as u8);
                } else if *pivot == here {   // No, is cell a pivot?
                    if unit_array.contains(&here) {
                        out_str.push('#' as u8);
                    } else {
                        out_str.push('*' as u8);
                    }
                } else {
                    if !unit_array.contains(&here) {
                        out_str.push('.' as u8);
                    }
                    else {
                        out_str.push('O' as u8);
                    }
                }
              }
              if y%2==0 {
                  out_str.push(' ' as u8);
              }
              out_str.push('|' as u8);
              out_str.push('\n' as u8);
            }
        }
        else {
            for y in 0..self.height as usize {
              out_str.push('|' as u8);
              if y%2==1 { out_str.push(' ' as u8); }

              for x in 0..self.width as usize {
                out_str.push(' ' as u8);

                if self.is_filled(Cell::new(x as i32, y as i32)) {
                    out_str.push('X' as u8);
                }
                else {
                    out_str.push('.' as u8);
                }
              }
              if y%2==0 {
                  out_str.push(' ' as u8);
              }
              out_str.push('|' as u8);
              out_str.push('\n' as u8);
            }
        }

        String::from_utf8(out_str).unwrap()
    }

  }

pub fn input_to_states(input: Input) -> Vec<State> {
    input.sourceSeeds.iter().map( |&s| {
        let mut seq: Vec<Unit> = Vec::with_capacity(input.sourceLength as usize);
        for i in get_source_order(s, input.sourceLength) {
            let mut unit = input.units[((i as usize) % input.units.len()) as usize].clone();
            let mut left = 1000;
            let mut right = 0;
            let mut top = 1000;
            for member in unit.members.iter() {
                if member.x < left { left = member.x }
                if member.x > right { right = member.x }
                if member.y < top { top = member.y }
            }
            if top > 0 { panic!("The top block has a y of {}, I don't know how to move it up to zero?!?!!??!?!?!", top); }
            let shift: i32 = (input.width - right - left) / 2;
            for i in (0..unit.members.len()) {
                unit.members[i].x += shift;
            }
            seq.push(unit);
        }
        let mut state = State::with_size(input.width, input.height);
        state.unit_sequence = seq;
        for &cell in input.filled.iter() {
            state.filled(cell);
        }
        state
    }).collect()
}

pub fn string_to_commands(s: &str) -> Vec<Command> {
    use Direction::*;
    let mut out = Vec::new();
    for c in s.chars() {
        match c.to_lowercase().next().unwrap() {
            'p' | '\'' | '!' | '.' | '0' | '3' => out.push(Command::Move(W)),
            'b' | 'c' | 'e' | 'f' | 'y' | '2' => out.push(Command::Move(E)),
            'a' | 'g' | 'h' | 'i' | 'j' | '4' => out.push(Command::Move(SW)),
            'l' | 'm' | 'n' | 'o' | ' ' | '5' => out.push(Command::Move(SE)),
            'd' | 'q' | 'r' | 'v' | 'z' | '1' => out.push(Command::Rotate(Clock::Wise)),
            'k' | 's' | 't' | 'u' | 'w' | 'x' => out.push(Command::Rotate(Clock::Counter)),
            '\t' | '\n' | '\r' => (),
            _ => unreachable!(),
        };
    }
    out
}

pub fn commands_to_string(cmds: Vec<Command>) -> String {
    cmds.iter().map(|&c| c.to_char()).collect()
}

pub fn get_source_order(seed: i32, num: i32) -> Vec<i32> {
    use std::num::Wrapping;
    fn unwrap<T>(x: Wrapping<T>) -> T {
        let Wrapping(x) = x;
        x
    }
    fn getout(x: Wrapping<u32>) -> i32 {
        unwrap((x>>16) & Wrapping(0x7fff)) as i32
    }

    let mut out_vec: Vec<i32> = Vec::with_capacity(num as usize);

    let multiplier: Wrapping<u32> = Wrapping(1103515245);
    let increment: Wrapping<u32> = Wrapping(12345);

    let mut x: Wrapping<u32> = Wrapping(seed as u32);

    out_vec.push(getout(x));

    for _ in 0..(num as usize) {
      x = multiplier*x + increment;
      out_vec.push(getout(x));
    }

    out_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_commands_works() {
        use Direction::*;
        assert_eq!(string_to_commands("pack"), vec![Command::Move(W),
                                                    Command::Move(SW),
                                                    Command::Move(E),
                                                    Command::Rotate(Clock::Counter)]);
        assert_eq!(string_to_commands("PACK"), vec![Command::Move(W),
                                                    Command::Move(SW),
                                                    Command::Move(E),
                                                    Command::Rotate(Clock::Counter)]);
        assert_eq!(string_to_commands("ei! "), vec![Command::Move(E),
                                                    Command::Move(SW),
                                                    Command::Move(W),
                                                    Command::Move(SE)]);
        assert_eq!(string_to_commands("Ei! "), vec![Command::Move(E),
                                                    Command::Move(SW),
                                                    Command::Move(W),
                                                    Command::Move(SE)]);
    }

    #[test]
    fn test_visualize() {
        let st0 = State::new();
        println!("out_str:\n{}\n", st0.visualize());
    }

	  #[test]
    fn test_psuedorandom() {
      let seed = 17;
      let n = 10;
        let sources = get_source_order(seed, n);
        let correct_sources = [0,24107,16552,12125,9427,13152,21440,3383,6873,16117];
        for i in 0..n as usize {
          assert_eq!(sources[i], correct_sources[i]);
        }
    }

}
