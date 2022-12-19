/* Advent of code 2022, Day 2: "Rock, Paper, Scissors"
 * Description: https://adventofcode.com/2022/day/2
 * Author: Tristan Andrus
 */

/**********************************
 * Short Description
 **********************************
 * Simulates a rock-paper scissors tournament from a given strategy guide.
 * Calculates the total score if the strategy guide is accurate.
 * Score for each round = <shape score> + (0 if you lost, 3 if draw, and 6 if you won)
 * Shape scores = 1 for Rock, 2 for Paper, and 3 for Scissors
 */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <strategy guide>", args[0]);
        std::process::exit(1);
    }

    let mut running_total = 0i32;
    let moves_in_game = parse_strategy_guide(&args[1]);
    for (opponent_move, my_move) in moves_in_game {
        running_total += calculate_points((opponent_move, my_move));
    }
    println!("Total points from this play: {}", running_total);
}

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum RoundResult {
    Win = 6,
    Draw = 3,
    Loss = 0
}

fn calculate_points(turn_moves: (Move, Move)) -> i32 {
    let mut result: RoundResult = RoundResult::Loss;
    // We leave out all the draws, since they will be caught by the `_` case
    match turn_moves {
        (Move::Rock,     Move::Paper) => result = RoundResult::Win,
        (Move::Rock,     Move::Scissors)  => result = RoundResult::Loss,
        (Move::Scissors, Move::Rock)  => result = RoundResult::Win,
        (Move::Scissors, Move::Paper) => result = RoundResult::Loss,
        (Move::Paper,    Move::Rock)     => result = RoundResult::Loss,
        (Move::Paper,    Move::Scissors) => result = RoundResult::Win,
        (_,_) => result = RoundResult::Draw,
    };
    result as i32 + turn_moves.1 as i32
}

fn string_to_move(input: &str) -> Move {
    match input {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => {
            eprintln!("Error! Unknown play");
            std::process::exit(1);
        }
    }
}

fn parse_strategy_guide_line(line: &String) -> (Move, Move) {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    (string_to_move(split_line[0]), string_to_move(split_line[1]),)
}

fn parse_strategy_guide(filename: &String) -> Vec<(Move, Move)> {
    let mut strategy_moves :Vec<(Move, Move)> = Vec::new();
    let file = File::open(filename).expect("Error: Could not read file");
    let buffered_contents = BufReader::new(file);
    let mut i = 1;
    for line in buffered_contents.lines() {
        match line {
            Ok(value) => {
                println!("{}, {}", i, value);
                i += 1;
                let play: (Move, Move) = parse_strategy_guide_line(&value);
                strategy_moves.push(play);
            }
            _ => eprintln!("No more input in file")
        };
    }
    strategy_moves
}
