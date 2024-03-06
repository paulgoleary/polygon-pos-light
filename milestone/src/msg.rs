
use crate::eth::EthAddress;

// type MsgMilestone struct {
// 	Proposer    types.HeimdallAddress `json:"proposer"`
// 	StartBlock  uint64                `json:"start_block"`
// 	EndBlock    uint64                `json:"end_block"`
// 	Hash        types.HeimdallHash    `json:"hash"`
// 	BorChainID  string                `json:"bor_chain_id"`
// 	MilestoneID string                `json:"milestone_id"`
// }

pub struct MsgMilestone {
    proposer: EthAddress,
    start_block: u64,
    end_block: u64,
    pub hash: [u8; 32],
    bor_chain_id: u64,
    milestone_id: [u8; 32]
}

// From Heimdall, checkpoint/types/msg_milestone.go
// Message is ABI-encoded for storage in transaction and signature
// // GetSideSignBytes returns side sign bytes
// func (msg MsgMilestone) GetSideSignBytes() []byte {
// 	// keccak256(abi.encoded(proposer, startBlock, endBlock, rootHash, accountRootHash, bor chain id))
// 	borChainID, _ := strconv.ParseUint(msg.BorChainID, 10, 64)
//
// 	return appendBytes32(
// 		msg.Proposer.Bytes(),
// 		new(big.Int).SetUint64(msg.StartBlock).Bytes(),
// 		new(big.Int).SetUint64(msg.EndBlock).Bytes(),
// 		msg.Hash.Bytes(),
// 		new(big.Int).SetUint64(borChainID).Bytes(),
// 		[]byte(msg.MilestoneID),
// 	)
// }

impl MsgMilestone {
    pub fn from_packed_bytes(abi_bytes: &[u8; 192]) -> Self {
        let mut u64_bytes = [0; 8];
        u64_bytes.copy_from_slice(&abi_bytes[56..64]);
        let start_block = u64::from_be_bytes(u64_bytes);
        u64_bytes.copy_from_slice(&abi_bytes[88..96]);
        let end_block = u64::from_be_bytes(u64_bytes);
        u64_bytes.copy_from_slice(&abi_bytes[152..160]);
        let bor_chain_id = u64::from_be_bytes(u64_bytes);
        let mut msg = MsgMilestone{
            proposer: EthAddress::default(),
            start_block,
            end_block,
            hash: [0; 32],
            bor_chain_id,
            milestone_id: [0; 32],
        };
        msg.proposer.copy_from_slice(&abi_bytes[12..32]);
        msg.hash.copy_from_slice(&abi_bytes[96..128]);
        msg.milestone_id.copy_from_slice(&abi_bytes[160..192]);
        msg
    }

    pub fn id_within(&self, id: u64) -> bool {
        id >= self.start_block && id <= self.end_block
    }
}

#[cfg(test)]
mod test {
    use crate::msg::MsgMilestone;

    #[test]
    fn test_decode() {
        let msg_bytes: [u8; 192] = hex::decode("0000000000000000000000004ad84f7014b7b44f723f284a85b166233797143900000000000000000000000000000000000000000000000000000000003b528500000000000000000000000000000000000000000000000000000000003b52926f73bdeda24c8d6b978628e10c425f5a8bbf181a547dafdf5eb156135626728e00000000000000000000000000000000000000000000000000000000000138820000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default().try_into().unwrap();
        let msg = MsgMilestone::from_packed_bytes(&msg_bytes);
        assert_eq!(msg.start_block, 3887749);
        assert_eq!(msg.end_block, 3887762);
        assert_eq!(hex::encode(msg.proposer), "4ad84f7014b7b44f723f284a85b1662337971439");
        assert_eq!(hex::encode(msg.hash), "6f73bdeda24c8d6b978628e10c425f5a8bbf181a547dafdf5eb156135626728e");
        assert_eq!(msg.bor_chain_id, 80002);
    }
}