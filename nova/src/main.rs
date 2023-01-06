const SOLVED_MAZE_F: &str = "../ts-solver/solutions/medium.soln.json";

use ff::PrimeField;
use num_bigint::BigInt;
use num_traits::Num;
use poseidon_rs::{Fr, Poseidon};
use serde::Deserialize;
use serde_json::json;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct SolvedMaze {
    maze: Vec<Vec<u32>>,
    height: u32,
    width: u32,
    solution: Vec<(u32, u32)>,
}

fn read_solved_maze(path: &str) -> SolvedMaze {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    serde_json::from_reader(rdr).unwrap()
}

fn main() {
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);

    let poseidon = Poseidon::new();
    let h_input = vec![Fr::from_str("1").unwrap()];
    let hex_out = poseidon.hash(h_input).unwrap().to_string();
    let stripped = hex_out
        .strip_prefix("Fr(0x")
        .unwrap()
        .strip_suffix(")")
        .unwrap();
    println!("{}", BigInt::from_str_radix(&stripped, 16).unwrap());

    // let mut private_inputs = Vec::new();
    // for i in 0..solved_maze.solution.len() {
    //     let mut priv_in = HashMap::from([
    //         (String::from("grid"), json!(solved_maze.maze)),
    //         (String::from("height"), json!(solved_maze.height)),
    //         (String::from("width"), json!(solved_maze.width)),
    //         (String::from("move"), json!(solved_maze.solution[i]))
    //     ]);
    //     private_inputs.push(priv_in);
    // }
}
