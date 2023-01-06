use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, F1, G1, G2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde::Deserialize;
use serde_json::json;

use std::{collections::HashMap, env::current_dir, fs::File, io::BufReader, time::Instant};

const R1CS_F: &str = "./circom/out/traversal.r1cs";
const WASM_F: &str = "./circom/out/traversal.wasm";
const SOLVED_MAZE_F: &str = "../ts-solver/solutions/small.soln.json";

#[derive(Deserialize, Debug)]
struct SolvedMaze {
    maze: Vec<Vec<u32>>,
    height: u32,
    width: u32,
    poseidon_vesta: String,
    solution: Vec<(i32, i32)>,
}

fn read_solved_maze(path: &str) -> SolvedMaze {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    serde_json::from_reader(rdr).unwrap()
}

fn main() {
    let root = current_dir().unwrap();
    let r1cs = load_r1cs(&root.join(R1CS_F));
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);
    let num_steps = solved_maze.solution.len() - 1;

    let mut private_inputs = Vec::new();
    for i in 0..num_steps {
        let dr = solved_maze.solution[i + 1].0 - solved_maze.solution[i].0;
        let dc = solved_maze.solution[i + 1].1 - solved_maze.solution[i].1;
        let mut priv_in = HashMap::from([
            (String::from("grid"), json!(solved_maze.maze)),
            (String::from("height"), json!(solved_maze.height)),
            (String::from("width"), json!(solved_maze.width)),
            (String::from("move"), json!([dr, dc])),
        ]);
        private_inputs.push(priv_in);
    }

    let pp = create_public_params(r1cs.clone());

    // Two roadblocks for hashing matrix 1) can't figure out poseidon hash on
    // vesta curve in rust and 2) F1::from caps out at u64
    let start_public_input = vec![F1::from(123), F1::from(0), F1::from(0), F1::from(1)];
    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        root.join(WASM_F),
        r1cs,
        private_inputs,
        start_public_input.clone(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // TODO: empty?
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        num_steps,
        start_public_input.clone(),
        z0_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());

    // produce a compressed SNARK
    println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let start = Instant::now();
    type S1 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G1>;
    type S2 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G2>;
    let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &recursive_snark);
    println!(
        "CompressedSNARK::prove: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    let compressed_snark = res.unwrap();

    // verify the compressed SNARK
    println!("Verifying a CompressedSNARK...");
    let start = Instant::now();
    let res = compressed_snark.verify(
        &pp,
        num_steps,
        start_public_input.clone(),
        z0_secondary,
    );
    println!(
        "CompressedSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
}
