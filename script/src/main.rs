//! A simple script to generate and verify the proof of a given program.

use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup a tracer for logging.
    utils::setup_tracer();

    // Create an input stream.
    let stdin = SP1Stdin::new();

    // Generate the proof for the given program.
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    let res = proof.stdout.read::<String>();
    println!("res: {}", res);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save the proof.
    proof
        .save("proof-with-pis.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}