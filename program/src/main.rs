//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use poly_milestone::proof;

pub fn main() {
    let headers: Vec<String> = sp1_zkvm::io::read();
    let check = proof::Proof::checkProof(headers);
    sp1_zkvm::io::write(&check);
}