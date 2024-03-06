use rs_merkle::Hasher as MerkleHasher;
use tiny_keccak::{Hasher, Keccak};

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

#[cfg(test)]
mod test {
    use rs_merkle::MerkleTree;
    use super::*;

    #[test]
    fn test_basic() {
        let leaf_hashes: Vec<[u8; 32]> = vec![
            hex::decode("fc905b8816642b177111968433a6aea8ea790ad2ea7c164de1625eaf01270f88").unwrap_or_default().try_into().unwrap(),
            hex::decode("cadfe86c5a7b1f839bfa2b7a11e5f3599b4d793daf50e690d1acbd8751175bfd").unwrap_or_default().try_into().unwrap()];
        let merkle_tree: MerkleTree<KeccakAlgorithm> = MerkleTree::<KeccakAlgorithm>::from_leaves(&leaf_hashes);
        let indices_to_prove = vec![0, 1];
        // let proof = merkle_tree.proof(&indices_to_prove);
        // 1b36fee251c3909dac8bf2f52beb249b34a49450ec77ebeb7eb3c8235d6244ae
        assert_eq!("1b36fee251c3909dac8bf2f52beb249b34a49450ec77ebeb7eb3c8235d6244ae",
                   merkle_tree.root_hex().unwrap_or_default());
    }
}