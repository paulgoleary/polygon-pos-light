use std::io::Bytes;
use num_bigint::BigInt;
use reth_primitives::Address;
use reth_primitives::revm_primitives::bitvec::view::BitViewSized;
use crate::hasher::keccak256;

// type Deposit struct {
// 	LeafType           uint8
// 	OriginalNetwork    uint
// 	OriginalAddress    common.Address
// 	Amount             *big.Int
// 	DestinationNetwork uint
// 	DestinationAddress common.Address
// 	DepositCount       uint
// 	BlockID            uint64
// 	BlockNumber        uint64
// 	NetworkID          uint
// 	TxHash             common.Hash
// 	Metadata           []byte
// 	// it is only used for the bridge service
// 	ReadyForClaim bool
// }

pub struct Deposit {
    leaf_type: u8,

    deposit_cnt: u64, // not included in hash ...

    pub orig_network: u32,
    pub orig_address: Address,

    pub dest_network: u32,
    pub dest_address: Address,

    pub amount: BigInt,

    metadata: Vec<u8>,
}

impl Deposit {

    pub fn leaf_hash(&self) -> [u8; 32] {

        let meta_hash = keccak256(&self.metadata);

        let mut amt_bytes = [0u8; 32];
        amt_bytes[32-self.amount.to_signed_bytes_be().len()..].copy_from_slice(self.amount.to_signed_bytes_be().as_slice());

        let to_hash = [
            self.leaf_type.as_raw_slice(),
            &u32::to_be_bytes(self.orig_network),
            &self.orig_address.as_slice(),
            &u32::to_be_bytes(self.dest_network),
            &self.dest_address.as_slice(),
            amt_bytes.as_slice(),
            &meta_hash
        ].concat();

        keccak256(&to_hash)
    }

    pub fn make_test(salt: u64) -> Self {
        let mut deposit = Deposit {
            leaf_type: 0,
            deposit_cnt: 0,
            orig_network: 0,
            orig_address: Address::default(),
            dest_network: 1,
            dest_address: Address::default(),
            amount: BigInt::default(),
            metadata: vec![0u8; 8],
        };

        let amount_bytes = hex::decode("8ac7230489e80000").unwrap_or_default();
        deposit.amount = BigInt::from_signed_bytes_be(amount_bytes.as_slice());
        // deposit.amount[24..].copy_from_slice(&amount_bytes);

        let dest_addr = hex::decode("c949254d682d8c9ad5682521675b8f43b102aec4").unwrap_or_default();

        deposit.dest_address.copy_from_slice(&dest_addr);
        deposit.orig_address.copy_from_slice(&dest_addr);

        deposit.metadata.copy_from_slice(salt.to_be_bytes().as_slice());

        deposit
    }
}

#[derive(Clone, Debug)]
pub struct ZKEVMDepositMerkle {
    deposit_count: usize,
    branch: [[u8; 32]; 32]
}

impl ZKEVMDepositMerkle {

    pub fn new() -> Self {
        ZKEVMDepositMerkle{
            deposit_count: 0,
            branch: [[0u8; 32]; 32]
        }
    }

    pub fn add_leaf(&mut self, leaf: [u8; 32]) {
        self.deposit_count += 1;
        let size = self.deposit_count;

        let mut node = [0u8; 32];
        node.copy_from_slice(leaf.as_slice());

        for height in 0..32 {
            if (size >> height & 1) == 1 {
                self.branch[height].copy_from_slice(node.as_slice());
                return;
            }
            let cat = [self.branch[height], node].concat();
            node.copy_from_slice(&keccak256(cat.as_slice()));
        }
        panic!("should not happen - should exit before here")
    }

    pub fn get_root(&self) -> [u8; 32] {
        let size = self.deposit_count;

        let mut node = [0u8; 32];

        let mut currentZeroHashHeight = [0u8; 32];

        for height in 0..32 {
            if (size >> height & 1) == 1 {
                let cat = [self.branch[height], node].concat();
                node.copy_from_slice(&keccak256(cat.as_slice()));
            } else {
                let cat = [node, currentZeroHashHeight].concat();
                node.copy_from_slice(&keccak256(cat.as_slice()));
            }

            let cat = [currentZeroHashHeight, currentZeroHashHeight].concat();
            currentZeroHashHeight.copy_from_slice(&keccak256(cat.as_slice()));
        }
        node
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;
    use reth_primitives::Address;
    use crate::hasher::keccak256;
    use crate::invariant::{Deposit, ZKEVMDepositMerkle};
    use std::str::FromStr;

    #[test]
    fn test_deposit_hash() {

        let mut deposit = Deposit {
            leaf_type: 0,
            deposit_cnt: 0,
            orig_network: 0,
            orig_address: Address::default(),
            dest_network: 1,
            dest_address: Address::default(),
            amount: BigInt::default(),
            metadata: vec![],
        };

        let amount_bytes = hex::decode("8ac7230489e80000").unwrap_or_default();
        deposit.amount = BigInt::from_signed_bytes_be(amount_bytes.as_slice());

        let dest_addr = hex::decode("c949254d682d8c9ad5682521675b8f43b102aec4").unwrap_or_default();
        deposit.dest_address.copy_from_slice(&dest_addr);

        let leaf_hash = deposit.leaf_hash();
        assert_eq!("22ed288677b4c2afd83a6d7d55f7df7f4eaaf60f7310210c030fd27adacbc5e0", hex::encode(leaf_hash));

        let mut dm = ZKEVMDepositMerkle::new();
        dm.add_leaf(leaf_hash);
        let dm_root = dm.get_root();
        assert_eq!("5ba002329b53c11a2f1dfe90b11e031771842056cf2125b43da8103c199dcd7f", hex::encode(dm_root));
    }

    #[test]
    fn test_go_compat() {
        let mut deposit = Deposit {
            leaf_type: 0,
            deposit_cnt: 0,
            orig_network: 0,
            orig_address: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            dest_network: 1,
            dest_address: Address::from_str("0x1DBA1131000664b884A1Ba238464159892252D3a").unwrap(),
            amount: BigInt::from_str("100000000000000000").unwrap(),
            metadata: vec![],
        };
        // {
        //    "removed":false,
        //    "block_number":16898815,
        //    "transaction_index":102,
        //    "log_index":217,
        //    "transaction_hash":"0x4b75f00d332b21a5ae368c6add032bc296138a0a9129470e394d8becc1e65db9",
        //    "event_type":2,
        //    "event_data":{
        //       "amount":100000000000000000,
        //       "depositCount":0,
        //       "destinationAddress":"0x1DBA1131000664b884A1Ba238464159892252D3a",
        //       "destinationNetwork":1,
        //       "leafType":0,
        //       "metadata":"",
        //       "originAddress":"0x0000000000000000000000000000000000000000",
        //       "originNetwork":0
        //    }
        // }

        let leaf_hash = deposit.leaf_hash();
        assert_eq!("b7cd745b9fc33c6e233768f51f262865c8cdff188d4e63c16709e389c11d5cd8", hex::encode(leaf_hash));

        let mut dm = ZKEVMDepositMerkle::new();
        dm.add_leaf(leaf_hash);
        let dm_root = dm.get_root();
        assert_eq!("927e6ceecb5b20935d26fbfc57002a59298b6e82640e8d652809e06854d7a81f", hex::encode(dm_root));
    }

    #[test]
    fn test_basic() {

        let mut dm = ZKEVMDepositMerkle{
            deposit_count: 0,
            branch: [[0u8; 32]; 32],
        };

        let d0 = Deposit::make_test(0);
        dm.add_leaf(d0.leaf_hash());

        let d1 = Deposit::make_test(1);
        dm.add_leaf(d1.leaf_hash());

        let root = dm.get_root();
    }
}
