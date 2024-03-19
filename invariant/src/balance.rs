use std::hash::Hash;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use rs_merkle::Hasher as MerkleHasher;
use tiny_keccak::{Hasher, Keccak};
use once_cell::sync::Lazy;
use ff::PrimeField;
use num_bigint::BigInt;
use reth_primitives::Address;
use crate::hasher::keccak256;
use crate::invariant::Deposit;

static POS: Lazy<Poseidon> = Lazy::new(|| {
    Poseidon::new()
});

#[derive(Clone)]
pub struct PoseidonAlgorithm {}

pub fn convert_u64(b: &[u8]) -> u64 {
    let mut buf = [0u8; 8];
    let len = 8.min(b.len());
    buf[8-len..].copy_from_slice(&b[..len]);
    u64::from_be_bytes(buf)
}

// TODO: endedness?
fn bytes_to_rep(b: &[u8]) -> FrRepr {
    let mut rep = [0 as u64; 4];
    if b.len() >= 24 {
        rep[3] = convert_u64(&b[24..]);
    }
    if b.len() >= 16 {
        rep[2] = convert_u64(&b[16..]);
    }
    if b.len() >= 8 {
        rep[1] = convert_u64(&b[8..]);
    }
    rep[0] = convert_u64(&b[0..]);
    FrRepr(rep)
}

impl MerkleHasher for PoseidonAlgorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        // naive impl of bytes -> field elements for poseidon
        let reps = data.chunks(31)
            .map(|b| bytes_to_rep(b))
            .map(|rep| Fr::from_raw_repr(rep).unwrap()).collect();
        let h = POS.hash(reps).unwrap();
        let h_rep = FrRepr::from(h);
        // TODO: endedness?
        let cat: Vec<u8> = [
            h_rep.0[0].to_be_bytes(),
            h_rep.0[1].to_be_bytes(),
            h_rep.0[2].to_be_bytes(),
            h_rep.0[3].to_be_bytes()].concat();
        cat.try_into().unwrap()
    }
}

struct BalanceWithdrawl {
    pub network: u32,
    pub address: Address,
    pub amount: BigInt,
}

impl BalanceWithdrawl {
    pub fn leaf_hash(&self) -> [u8; 32] {

        let mut amt_bytes = [0u8; 32];
        amt_bytes[32-self.amount.to_signed_bytes_be().len()..].copy_from_slice(self.amount.to_signed_bytes_be().as_slice());

        let to_hash = [
            &u32::to_be_bytes(self.network),
            self.address.as_slice(),
            amt_bytes.as_slice(),
        ].concat();

        PoseidonAlgorithm::hash(&to_hash)
    }

    pub fn make_test(salt: u64) -> Self {
        let mut withdrawl = BalanceWithdrawl {
            network: 1,
            address: Address::default(),
            amount: BigInt::default(),
        };

        let amount_bytes = hex::decode("8ac7230489e80000").unwrap_or_default();
        let salted_amount = [salt.to_be_bytes(), salt.to_be_bytes()].concat();
        withdrawl.amount = BigInt::from_signed_bytes_be(salted_amount.as_slice());

        let dest_addr = hex::decode("c949254d682d8c9ad5682521675b8f43b102aec4").unwrap_or_default();
        withdrawl.address.copy_from_slice(&dest_addr);

        withdrawl
    }
}

#[cfg(test)]
mod test {
    use rs_merkle::{Hasher, MerkleTree};
    use crate::balance::PoseidonAlgorithm;
    use poseidon_rs::{Fr, FrRepr, Poseidon};
    use ff::PrimeField;

    #[test]
    fn test_merkle() {
        let leaf_hashes: Vec<[u8; 32]> = vec![
            hex::decode("fc905b8816642b177111968433a6aea8ea790ad2ea7c164de1625eaf01270f88").unwrap_or_default().try_into().unwrap(),
            hex::decode("cadfe86c5a7b1f839bfa2b7a11e5f3599b4d793daf50e690d1acbd8751175bfd").unwrap_or_default().try_into().unwrap()];

        // PoseidonAlgorithm::hash(leaf_hashes[0].as_slice());
        // PoseidonAlgorithm::hash(leaf_hashes[1].as_slice());

        let mut mt: MerkleTree<PoseidonAlgorithm> = Default::default();
        mt.insert(leaf_hashes[0]);
        mt.insert(leaf_hashes[1]);
        mt.commit();

        assert_eq!("bf8848b60e2bdbb875c48336c4d3652ea462cc4779ad9ff51be6cfa9e78d0f81",
                   mt.root_hex().unwrap_or_default());
    }

    // 29176100ea a962bdc1fe 6c654d6a3c 130e96a4d1 168b33848b 897dc50282 0133

    #[test]
    fn test_poseidon() {
        let b1: Fr = Fr::from_str("1").unwrap();

        let mut rep = [0 as u64; 4];
        let b2 = Fr::from_raw_repr(poseidon_rs::FrRepr(rep));

        let poseidon = Poseidon::new();

        let big_arr: Vec<Fr> = vec![b1];
        // let mut big_arr: Vec<Fr> = Vec::new();
        // big_arr.push(b1.clone());
        let h = poseidon.hash(big_arr).unwrap();
        assert_eq!(
            h.to_string(),
            "Fr(0x29176100eaa962bdc1fe6c654d6a3c130e96a4d1168b33848b897dc502820133)" // "18586133768512220936620570745912940619677854269274689475585506675881198879027"
        );
    }
}
