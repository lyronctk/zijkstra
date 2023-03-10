/*
 * Recursively proves knowledge of the shortest path through a maze using a
 * combination of Nova and Spartan w/ IPA-PC. Concretely, this produces a SNARK
 * proof that shows "I know a path leading to cell [X, Y] at cost C" and can be
 * verified in sub-linear time.
 */

use nova_scotia::{
    circom::{
        circuit::{CircomCircuit, R1CS},
        reader::load_r1cs,
    },
    create_public_params, create_recursive_circuit, FileLocation, F1, F2, G1,
    G2,
};
use nova_snark::{
    traits::{circuit::TrivialTestCircuit, Group},
    CompressedSNARK, PublicParams, RecursiveSNARK,
};
use num_bigint::BigInt;
use num_traits::Num;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap, env::current_dir, fs, fs::File, io::BufReader,
    path::Path, path::PathBuf, time::Instant,
};

type C1 = CircomCircuit<<G1 as Group>::Scalar>;
type C2 = TrivialTestCircuit<<G2 as Group>::Scalar>;
type EE1 = nova_snark::provider::ipa_pc::EvaluationEngine<G1>;
type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<G2>;
type S1 = nova_snark::spartan::RelaxedR1CSSNARK<G1, EE1>;
type S2 = nova_snark::spartan::RelaxedR1CSSNARK<G2, EE2>;

const R1CS_F: &str = "./circom/out/traversal.r1cs";
const WASM_F: &str = "./circom/out/traversal.wasm";
const SOLVED_MAZE_F: &str = "../ts-solver/solutions/small.soln.json";
const PROOF_OUT_F: &str = "./out/spartan_proof.json";

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

#[derive(Serialize)]
struct StringifiedSpartanProof {
    nifs_primary: String,
    f_W_snark_primary: String,
    nifs_secondary: String,
    f_W_snark_secondary: String,
    zn_primary: String,
    zn_secondary: String,
}

fn read_solved_maze(path: &str) -> SolvedMaze {
    let f = File::open(path).unwrap();
    let rdr = BufReader::new(f);
    println!("- Done");
    serde_json::from_reader(rdr).unwrap()
}

/*
 * Generates public parameters for Nova.
 */
fn setup(r1cs: &R1CS<F1>) -> PublicParams<G1, G2, C1, C2> {
    let pp = create_public_params(r1cs.clone());

    println!(
        "- Number of constraints per step (primary): {}",
        pp.num_constraints().0
    );
    println!(
        "- Number of constraints per step (secondary): {}",
        pp.num_constraints().1
    );

    pp
}

/*
 * Constructs the inputs necessary for recursion. This includes 1) private
 * inputs for every step, and 2) initial public inputs for the first step of the
 * primary & secondary circuits.
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

/*
 * Converts hex elements (field elements in this case) into decimal strings for
 * logging.
 */
fn hex_to_decimal_str(v: Vec<F1>) -> Vec<String> {
    let stripped = v
        .iter()
        .map(|&x| format!("{:?}", x).strip_prefix("0x").unwrap().to_string())
        .collect::<Vec<String>>();

    stripped
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
        FileLocation::PathBuf(witness_gen),
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
        hex_to_decimal_str(res.unwrap().0)
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
    proof_f: &str,
) -> CompressedSNARK<G1, G2, C1, C2, S1, S2> {
    println!("- Generating");
    let start = Instant::now();
    let res =
        CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &recursive_snark);
    assert!(res.is_ok());
    println!("- Done ({:?})", start.elapsed());
    let compressed_snark = res.unwrap();

    let prf = StringifiedSpartanProof {
        nifs_primary: format!("{:?}", compressed_snark.nifs_primary),
        f_W_snark_primary: format!("{:?}", compressed_snark.f_W_snark_primary),
        nifs_secondary: format!("{:?}", compressed_snark.nifs_secondary),
        f_W_snark_secondary: format!(
            "{:?}",
            compressed_snark.f_W_snark_secondary
        ),
        zn_primary: format!("{:?}", compressed_snark.zn_primary),
        zn_secondary: format!("{:?}", compressed_snark.zn_secondary),
    };
    let prf_json = serde_json::to_string(&prf).unwrap();
    fs::write(&proof_f, prf_json).unwrap();

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
    let r1cs = load_r1cs(&FileLocation::PathBuf(root.join(R1CS_F)));
    let witness_gen = root.join(WASM_F);

    let start = Instant::now();
    println!("== Loading solved maze");
    let solved_maze = read_solved_maze(SOLVED_MAZE_F);
    let num_steps = solved_maze.solution.len() - 1;
    println!("==");

    println!("== Creating circuit public parameters");
    let pp = setup(&r1cs);
    println!("==");

    println!("== Constructing inputs");
    let inputs = construct_inputs(&solved_maze, num_steps);
    println!("==");

    println!("== Executing recursion using Nova");
    let recursive_snark = recursion(witness_gen, r1cs, &inputs, &pp, num_steps);
    println!("==");

    println!("== Producing a CompressedSNARK using Spartan w/ IPA-PC");
    let _compressed_snark =
        spartan(&pp, recursive_snark, num_steps, &inputs, PROOF_OUT_F);
    println!("==");
    println!("** Total time to completion: ({:?})", start.elapsed());
}
