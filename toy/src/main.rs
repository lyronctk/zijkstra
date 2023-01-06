use std::{collections::HashMap, env::current_dir, time::Instant};

use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, F1, G1, G2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use serde_json::json;

#[derive(Debug)]
struct CustomPublicInput {
    a: Vec<F1>,
    b: F1,
}

#[derive(Debug)]
struct CustomPrivateInput {
    c: Vec<Vec<F1>>,
}

fn main() {
    let iteration_count = 3;
    let root = current_dir().unwrap();

    let circuit_file = root.join("src/toy.r1cs");
    let r1cs = load_r1cs(&circuit_file);
    let witness_generator_file = root.join("src/toy.wasm");

    let mut priv_inputs: Vec<CustomPrivateInput> = Vec::new();
    for i in 0..iteration_count {
        let mut priv_in = CustomPrivateInput { c: vec![
            vec![F1::from(i), F1::from(i)],
            vec![F1::from(i * 2), F1::from(i * 2)],
        ] };
        priv_inputs.push(priv_in);
    }

    let start_pub_input = CustomPublicInput {
        a: vec![F1::from(0), F1::from(0)],
        b: F1::from(1)
    };

    println!("{:?}", priv_inputs);
    println!("{:?}", start_pub_input);
    return;

    // let start_public_input = vec![F1::from(10), F1::from(10)];

    // let pp = create_public_params(r1cs.clone());

    // println!(
    //     "Number of constraints per step (primary circuit): {}",
    //     pp.num_constraints().0
    // );
    // println!(
    //     "Number of constraints per step (secondary circuit): {}",
    //     pp.num_constraints().1
    // );

    // println!("Creating a RecursiveSNARK...");
    // let start = Instant::now();
    // let recursive_snark = create_recursive_circuit(
    //     witness_generator_file,
    //     r1cs,
    //     private_inputs,
    //     start_public_input.clone(),
    //     &pp,
    // )
    // .unwrap();
    // println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // // TODO: empty?
    // let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // // verify the recursive SNARK
    // println!("Verifying a RecursiveSNARK...");
    // let start = Instant::now();
    // let res = recursive_snark.verify(
    //     &pp,
    //     iteration_count,
    //     start_public_input.clone(),
    //     z0_secondary.clone(),
    // );
    // println!(
    //     "RecursiveSNARK::verify: {:?}, took {:?}",
    //     res,
    //     start.elapsed()
    // );
    // assert!(res.is_ok());

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

    println!("== COMPRESSED SNARK PROOF");
    println!("{:?}", compressed_snark.f_W_snark_primary);
    println!("==");

    // verify the compressed SNARK
    println!("Verifying a CompressedSNARK...");
    let start = Instant::now();
    let res = compressed_snark.verify(
        &pp,
        iteration_count,
        start_public_input.clone(),
        z0_secondary,
    );
    // println!(
    //     "CompressedSNARK::verify: {:?}, took {:?}",
    //     res.is_ok(),
    //     start.elapsed()
    // );
    // assert!(res.is_ok());
}
