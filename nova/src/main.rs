const SOLVED_MAZE_F: &str = "../ts-solver/solutions/medium.soln.json";

use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct SolvedMaze {
    maze: Vec<Vec<u32>>,
    solution: Vec<(u32, u32)>
}

fn read_solved_maze(path: &str) -> SolvedMaze {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    serde_json::from_reader(rdr).unwrap()
}

fn main() {
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);
    println!("{:?}", solved_maze);
}
