use std::env;
use std::process::exit;

use common::read_lines;

use std::{num::ParseIntError, error::Error};

const RED_CUBES:usize = 12;
const GREEN_CUBES:usize = 13;
const BLUE_CUBES:usize = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file provided");
        exit(1);
    }
    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        let mut result: u16 = 0;
        for line in lines {
            if let Ok(value) = line.map( |txt| {
                Game::parse(txt).unwrap()
            }) {
                if value.is_possible() {
                    result += value.number;
                }
            }
        }
        println!("The sum of all games is {}", result);
    }

    if let Ok(lines) = read_lines(filename) {
        let mut result: u32 = 0;
        for line in lines {
            if let Ok(value) = line.map( |txt| {
                Game::parse(txt).unwrap()
            }) {
                result += value.minimum().power();
            }
        }
        println!("The sum of all cubes is {}", result);
    }

    Ok(())
}


#[derive(Debug, PartialEq, Eq)]
struct Game {
    number: u16,
    picks: Vec<Pick>
}

impl Game {
    fn parse(line: String) ->Result<Self, Box<dyn Error>> {
        let el: Vec<_> = line.split(':').collect();
        let game = if let Some((first, elements)) = el.split_first() {
            let _gn = extract_game_number(first).unwrap();
            let mut _pcs: Vec<Pick> = Vec::new();
            elements[0].split(';')
                .for_each(|text| {
                    let pick = extract_pick(text).unwrap();
                    _pcs.push(pick);
                });
            
            Ok( Game {
                    number: _gn,
                    picks: _pcs
                })
        } else { panic!("boom") };
        game
    }

    fn is_possible(&self) -> bool {
        return self.picks.iter().all(|p| {
            p.red <= RED_CUBES && p.green <= GREEN_CUBES && p.blue <= BLUE_CUBES
        });
    }

    fn minimum(&self) -> Pick {
        let mut out = Pick {
            red: 0,
            blue: 0, 
            green: 0
        };
        self.picks.iter().for_each(|p| {
          if p.red > out.red {
            out.red = p.red
          }  
          if p.green > out.green {
            out.green = p.green
          }  
          if p.blue > out.blue {
            out.blue = p.blue
          }  
        });
        out    
    }
}

fn extract_game_number(text: &str) -> Result<u16, ParseIntError> {
    return text.replace("Game ", "").parse::<u16>();
}

fn extract_pick(text: &str) -> Result<Pick, Box<dyn Error>> {
    let mut red = 0usize;
    let mut green = 0usize;
    let mut blue = 0usize;

    text.split(',')
    .filter(|e| !e.is_empty())
    .for_each(|el| {
        let mut it = el.split_whitespace();
        let n = usize::from_str_radix(it.next().unwrap(), 10).unwrap();
        let color = it.next().unwrap();
        match color {
            "red" => red = n,
            "green" => green = n,
            "blue" => blue = n,
            _ => panic!("Unknown color")
        }
    });
    
    Ok(Pick {
        red: red,
        green: green, 
        blue: blue
    })
}


#[derive(Debug, PartialEq, Eq)]
struct Pick {
    red: usize,
    green: usize,
    blue: usize
}

impl Pick {
    
    fn power(&self) ->u32 {
        return self.red as u32 * self.green as u32 * self.blue as u32;
    }
}

#[cfg(test)]
mod tests {

    use crate::{Game, Pick};

    #[test]
    fn test() {
        let txt = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue".to_owned();

        let game = Game::parse(txt);

        assert_eq!(game.unwrap(), Game {
            number: 1,
            picks: vec![
                Pick{ red: 2, green: 2,  blue: 0},
                Pick{ red: 1, green: 1,  blue: 2},
                Pick{ red: 3, green: 3,  blue: 3},
                Pick{ red: 7, green: 3,  blue: 1},
                Pick{ red: 5, green: 3,  blue: 1}
            ]
        });

    }
}