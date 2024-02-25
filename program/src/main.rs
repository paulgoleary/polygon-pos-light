//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use rs_merkle::{
    algorithms::Sha256, proof_serializers::DirectHashesOrder, Error, Hasher, MerkleProof, MerkleTree,
};
use poly_checkpoint::hasher::KeccakAlgorithm;

pub struct TestData {
    pub leaf_values: Vec<String>,
    pub expected_root_hex: String,
    pub leaf_hashes: Vec<[u8; 32]>,
}

pub fn setup() -> TestData {
    let leaf_values = ["a", "b", "c", "d", "e", "f"];
    let expected_root_hex = "1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2";
    let leaf_hashes = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    TestData {
        leaf_values: leaf_values.iter().cloned().map(String::from).collect(),
        leaf_hashes,
        expected_root_hex: String::from(expected_root_hex),
    }
}

pub fn main() {

    let test_data = setup();
    let expected_root = test_data.expected_root_hex.clone();
    let leaf_hashes = &test_data.leaf_hashes;
    let indices_to_prove = vec![3, 4];

    let leaves_to_prove: Vec<[u8; 32]> = indices_to_prove
        .iter()
        .map(|i| *leaf_hashes.get(*i).unwrap())
        .collect();

    let merkle_tree: MerkleTree<KeccakAlgorithm> = MerkleTree::<KeccakAlgorithm>::from_leaves(&test_data.leaf_hashes);
    let proof = merkle_tree.proof(&indices_to_prove);
    let extracted_root = proof.root_hex(
        &indices_to_prove,
        &leaves_to_prove,
        test_data.leaf_values.len(),
    ).unwrap_or_default();

    sp1_zkvm::io::write(&extracted_root);

    // sp1_zkvm::io::write(&a);
    // sp1_zkvm::io::write(&b);
}