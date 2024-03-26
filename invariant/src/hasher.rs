use tiny_keccak::{Hasher, Keccak};
use rs_merkle::Hasher as MerkleHasher;

// TODO: move to common or something?
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut keccak256 = Keccak::v256();
    keccak256.update(data);
    let mut output = [0u8; 32];
    keccak256.finalize(&mut output);
    output
}

#[derive(Clone)]
pub struct KeccakAlgorithm {}

impl MerkleHasher for KeccakAlgorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak256(data)
    }
}

