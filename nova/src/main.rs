use nova_scotia::{
    circom::circuit::CircomCircuit, circom::circuit::R1CS,
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit,
    F1, F2, G1, G2, C1, C2
};
use nova_snark::{
    traits::circuit::TrivialTestCircuit, CompressedSNARK,
    PublicParams,
};
use serde::Deserialize;
use serde_json::{json, Value};

use std::{
    collections::HashMap, env::current_dir, fs::File, io::BufReader,
    path::PathBuf, time::Instant,
};

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

struct RecursionInputs {
    all_private: Vec<HashMap<String, Value>>,
    start_pub_primary: Vec<F1>,
    start_pub_secondary: Vec<F2>,
}

fn read_solved_maze(path: &str) -> SolvedMaze {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    serde_json::from_reader(rdr).unwrap()
}

fn construct_inputs(
    solved_maze: &SolvedMaze,
    num_steps: usize,
) -> RecursionInputs {
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

    // [TODO] Checkings the grid hash currently disabled. Need to run poseidon
    //        hash on the Vesta curve. Also need to load into F1, which only has
    //        From() trait implemented for u64.
    let z0_primary = vec![
        F1::from(123),
        F1::from(0),
        F1::from(0),
        F1::from(solved_maze.maze[0][0] as u64),
    ];
    let z0_secondary = vec![F2::zero()];

    RecursionInputs {
        all_private: private_inputs,
        start_pub_primary: z0_primary,
        start_pub_secondary: z0_secondary,
    }
}

fn recursion(
    witness_gen: PathBuf,
    r1cs: R1CS<F1>,
    inputs: RecursionInputs,
    pp: PublicParams<G1, G2, CircomCircuit<F1>, TrivialTestCircuit<F2>>,
    num_steps: usize,
) -> RecursiveSNARK<G1, G2, C1, C2> {
    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        witness_gen,
        r1cs,
        inputs.all_private,
        inputs.start_pub_primary.clone(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        num_steps,
        inputs.start_pub_primary.clone(),
        inputs.start_pub_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());
}

fn main() {
    let root = current_dir().unwrap();
    let r1cs = load_r1cs(&root.join(R1CS_F));
    let witness_gen = root.join(WASM_F);

    println!("== Loading the solved maze");
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);
    let num_steps = solved_maze.solution.len() - 1;
    println!("==");

    println!("== Creating circuit public parameters");
    let pp = create_public_params(r1cs.clone());
    println!("==");

    println!("== Constructing inputs");
    let inputs = construct_inputs(&solved_maze, num_steps);
    println!("==");

    println!("== Executing recursion w/ Nova");
    let recursive_snark = recursion(witness_gen, r1cs, inputs, pp, num_steps);
    println!("==");

    // produce a compressed SNARK
    println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let start = Instant::now();
    type S1 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G1>;
    type S2 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G2>;
    let res =
        CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &recursive_snark);
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
