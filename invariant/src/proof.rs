use rs_merkle::MerkleTree;
// use crate::balance::PoseidonAlgorithm;
use crate::hasher::KeccakAlgorithm;
use crate::invariant::{Deposit, ZKEVMDepositMerkle};

pub struct Proof {}

impl Proof {
    pub fn do_proof(prev_state: &ZKEVMDepositMerkle,
                    prev_root: [u8; 32], new_root: [u8; 32],
                    deposits: Vec<Deposit>) -> bool {

        // confirm previous state matches confirmed previous root
        if prev_state.get_root() != prev_root {
            return false;
        }

        let new_state = deposits.iter().fold(prev_state.clone(), |mut acc, dep| {
            acc.add_leaf(dep.leaf_hash());
            acc
        });

        // confirm that the new root matches tree with new deposits
        if new_state.get_root() != new_root {
            return false;
        }

        true
    }

    pub fn do_balance_test(num_leafs: u64) {
        let mut mt: MerkleTree<KeccakAlgorithm> = Default::default();

        for x in 0..num_leafs {
            let d = Deposit::make_test(x);
            mt.insert(d.leaf_hash());
        }

        mt.commit();
    }
}

#[cfg(test)]
mod test {
    use crate::invariant::{Deposit, ZKEVMDepositMerkle};
    use crate::proof::Proof;

    #[test]
    fn test_proof() {
        let mut prev_dm = ZKEVMDepositMerkle::new();
        for i in 0..10 {
            let d = Deposit::make_test(i);
            prev_dm.add_leaf(d.leaf_hash());
        }
        let mut new_dm = prev_dm.clone();
        let mut new_deps: Vec<Deposit> = Vec::new();
        for i in 10..20 {
            let d = Deposit::make_test(i);
            new_dm.add_leaf(d.leaf_hash());
            new_deps.push(d);
        }
        let valid = Proof::do_proof(&prev_dm, prev_dm.get_root(), new_dm.get_root(), new_deps);
        assert!(valid);
    }
}