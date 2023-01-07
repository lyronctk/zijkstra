use nova_scotia::{
    circom::circuit::CircomCircuit, circom::circuit::R1CS,
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit,
    F1, F2, G1, G2,
};
use nova_snark::{
    traits::circuit::TrivialTestCircuit, traits::Group, CompressedSNARK,
    PublicParams, RecursiveSNARK,
};
use num_bigint::BigInt;
use num_traits::Num;
use serde::Deserialize;
use serde_json::{json, Value};

use std::{
    collections::HashMap, env::current_dir, fs::File, io::BufReader,
    path::PathBuf, time::Instant,
};

type C1 = CircomCircuit<<G1 as Group>::Scalar>;
type C2 = TrivialTestCircuit<<G2 as Group>::Scalar>;
type S1 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G1>;
type S2 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G2>;

const R1CS_F: &str = "./circom/out/traversal.r1cs";
const WASM_F: &str = "./circom/out/traversal.wasm";
const SOLVED_MAZE_F: &str = "../ts-solver/solutions/small.soln.json";

#[derive(Deserialize, Debug)]
struct SolvedMaze {
    maze: Vec<Vec<u32>>,
    height: u32,
    width: u32,
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
    println!("- Done");
    serde_json::from_reader(rdr).unwrap()
}

/*
 * Constructs the inputs necessary for recursion. Concretely, this includes
 * 1) private inputs for every step, and 2) initial public inputs for the
 * first step of the primary & secondary circuits.
 */
fn construct_inputs(
    solved_maze: &SolvedMaze,
    num_steps: usize,
) -> RecursionInputs {
    let mut private_inputs = Vec::new();
    for i in 0..num_steps {
        let dr = solved_maze.solution[i + 1].0 - solved_maze.solution[i].0;
        let dc = solved_maze.solution[i + 1].1 - solved_maze.solution[i].1;
        let priv_in = HashMap::from([
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

    // Secondary circuit is TrivialTestCircuit, filler val
    let z0_secondary = vec![F2::zero()];

    println!("- Done");
    RecursionInputs {
        all_private: private_inputs,
        start_pub_primary: z0_primary,
        start_pub_secondary: z0_secondary,
    }
}

fn Fq_to_decimal_str(v: Vec<F1>) -> Vec<String> {
    let hexified = v
        .iter()
        .map(|&x| format!("{:?}", x).strip_prefix("0x").unwrap().to_string())
        .collect::<Vec<String>>();

    hexified
        .iter()
        .map(|x| BigInt::from_str_radix(x, 16).unwrap().to_str_radix(10))
        .collect()
}

/*
 * Uses Nova's folding scheme to produce a single relaxed R1CS instance that,
 * when satisfied, proves the proper execution of every step in the recursion.
 * Can be thought of as a pre-processing step for the final SNARK.
 */
fn recursion(
    witness_gen: PathBuf,
    r1cs: R1CS<F1>,
    inputs: &RecursionInputs,
    pp: &PublicParams<G1, G2, C1, C2>,
    num_steps: usize,
) -> RecursiveSNARK<G1, G2, C1, C2> {
    println!("- Creating RecursiveSNARK");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        witness_gen,
        r1cs,
        inputs.all_private.clone(),
        inputs.start_pub_primary.clone(),
        &pp,
    )
    .unwrap();
    println!("- Done ({:?})", start.elapsed());

    println!("- Verifying RecursiveSNARK");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        num_steps,
        inputs.start_pub_primary.clone(),
        inputs.start_pub_secondary.clone(),
    );
    assert!(res.is_ok());
    println!(
        "- Output of final step: {:?}",
        Fq_to_decimal_str(res.unwrap().0)
    );
    println!("- Done ({:?})", start.elapsed());

    recursive_snark
}

/*
 * Uses Spartan w/ IPA-PC to prove knowledge of the output of Nova (a satisfied
 * relaxed R1CS instance) in a proof that can be verified with sub-linear cost.
 */
fn spartan(
    pp: &PublicParams<G1, G2, C1, C2>,
    recursive_snark: RecursiveSNARK<G1, G2, C1, C2>,
    num_steps: usize,
    inputs: &RecursionInputs,
) -> CompressedSNARK<G1, G2, C1, C2, S1, S2> {
    println!("- Generating");
    let start = Instant::now();
    type S1 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G1>;
    type S2 = nova_snark::spartan_with_ipa_pc::RelaxedR1CSSNARK<G2>;
    let res =
        CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &recursive_snark);
    assert!(res.is_ok());
    println!("- Done ({:?})", start.elapsed());
    let compressed_snark = res.unwrap();
    println!("- Proof: {:?}", compressed_snark.f_W_snark_primary);

    println!("- Verifying");
    let start = Instant::now();
    let res = compressed_snark.verify(
        &pp,
        num_steps,
        inputs.start_pub_primary.clone(),
        inputs.start_pub_secondary.clone(),
    );
    assert!(res.is_ok());
    println!("- Done ({:?})", start.elapsed());

    compressed_snark
}

fn main() {
    let root = current_dir().unwrap();
    let r1cs = load_r1cs(&root.join(R1CS_F));
    let witness_gen = root.join(WASM_F);

    println!("== Loading solved maze");
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);
    let num_steps = solved_maze.solution.len() - 1;
    println!("==");

    println!("== Creating circuit public parameters");
    let pp = create_public_params(r1cs.clone());
    println!("- Done");
    println!("==");

    println!("== Constructing inputs");
    let inputs = construct_inputs(&solved_maze, num_steps);
    println!("==");

    println!("== Executing recursion using Nova");
    let recursive_snark = recursion(witness_gen, r1cs, &inputs, &pp, num_steps);
    println!("==");

    println!("== Producing a CompressedSNARK using Spartan w/ IPA-PC");
    let _compressed_snark = spartan(&pp, recursive_snark, num_steps, &inputs);
    println!("==")
}
