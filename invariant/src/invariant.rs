use std::io::Bytes;
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

struct Deposit {
    leaf_type: u8,

    orig_network: u32,
    orig_address: [u8; 20],

    dest_network: u32,
    dest_address: [u8; 20],

    amount: [u8; 32],

    metadata: Vec<u8>,
}

impl Deposit {

    fn leaf_hash(&self) -> [u8; 32] {
        
        // 	var res [KeyLen]byte
        // 	origNet := make([]byte, 4) //nolint:gomnd
        // 	binary.BigEndian.PutUint32(origNet, uint32(deposit.OriginalNetwork))
        // 	destNet := make([]byte, 4) //nolint:gomnd
        // 	binary.BigEndian.PutUint32(destNet, uint32(deposit.DestinationNetwork))
        // 	var buf [KeyLen]byte
        // 	metaHash := keccak256.Hash(deposit.Metadata)
        // 	copy(res[:], keccak256.Hash([]byte{deposit.LeafType}, origNet, deposit.OriginalAddress[:], destNet, deposit.DestinationAddress[:], deposit.Amount.FillBytes(buf[:]), metaHash))
        // 	return res'

        let meta_hash = keccak256(&self.metadata);

        let to_hash = [
            self.leaf_type.as_raw_slice(),
            &u32::to_be_bytes(self.orig_network),
            &self.orig_address,
            &u32::to_be_bytes(self.dest_network),
            &self.dest_address,
            &self.amount,
            &meta_hash
        ].concat();

        keccak256(&to_hash)
    }
}

struct ZKEVMDepositMerkle {
    deposit_count: usize,
    branch: [[u8; 32]; 32]
}

impl ZKEVMDepositMerkle {

    fn new() -> Self {
        ZKEVMDepositMerkle{
            deposit_count: 0,
            branch: [[0u8; 32]; 32]
        }
    }

    fn add_leaf(&mut self, leaf: [u8; 32]) {
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

    fn get_root(&self) -> [u8; 32] {
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
    use crate::hasher::keccak256;
    use crate::invariant::{Deposit, ZKEVMDepositMerkle};

    #[test]
    fn test_deposit_hash() {

        // 	deposit := &etherman.Deposit{
        // 		OriginalNetwork:    0,
        // 		OriginalAddress:    common.Address{},
        // 		Amount:             amount,
        // 		DestinationNetwork: 1,
        // 		DestinationAddress: common.HexToAddress("0xc949254d682d8c9ad5682521675b8f43b102aec4"),
        // 		BlockNumber:        0,
        // 		DepositCount:       0,
        // 		Metadata:           []byte{},
        // 	}

        let mut deposit = Deposit{
            leaf_type: 0,
            orig_network: 0,
            orig_address: [0u8; 20],
            dest_network: 1,
            dest_address: [0u8; 20],
            amount: [0u8; 32],
            metadata: vec![],
        };

        let amount_bytes = hex::decode("8ac7230489e80000").unwrap_or_default();
        deposit.amount[24..].copy_from_slice(&amount_bytes);

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
    fn test_basic() {

        let mut dm = ZKEVMDepositMerkle{
            deposit_count: 0,
            branch: [[0u8; 32]; 32],
        };

        let mut test_leaf = [0u8; 32];

        test_leaf[0] = 1;
        dm.add_leaf(test_leaf);

        test_leaf[0] = 2;
        dm.add_leaf(test_leaf);

        let root = dm.get_root();
    }
}
