
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
    startBlock: u64,
    endBlock: u64,
    hash: [u8; 32],
    borChainID: u64,
    milestoneId: [u8; 32]
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
    fn fromABIBytes(abiBytes: &[u8; 192]) -> Self {
        let mut u64_bytes = [0; 8];
        u64_bytes.copy_from_slice(&abiBytes[56..64]);
        let startBlock = u64::from_be_bytes(u64_bytes);
        u64_bytes.copy_from_slice(&abiBytes[88..96]);
        let endBlock = u64::from_be_bytes(u64_bytes);
        u64_bytes.copy_from_slice(&abiBytes[152..160]);
        let borChainId = u64::from_be_bytes(u64_bytes);
        let mut msg = MsgMilestone{
            proposer: EthAddress::default(),
            startBlock: startBlock,
            endBlock: endBlock,
            hash: [0; 32],
            borChainID: borChainId,
            milestoneId: [0; 32],
        };
        msg.proposer.copy_from_slice(&abiBytes[12..32]);
        msg.hash.copy_from_slice(&abiBytes[96..128]);
        msg.milestoneId.copy_from_slice(&abiBytes[160..192]);
        msg
    }
}

#[cfg(test)]
mod test {
    use crate::msg::MsgMilestone;

    #[test]
    fn test_decode() {
        let msg_bytes: [u8; 192] = hex::decode("0000000000000000000000004ad84f7014b7b44f723f284a85b166233797143900000000000000000000000000000000000000000000000000000000003b528500000000000000000000000000000000000000000000000000000000003b52926f73bdeda24c8d6b978628e10c425f5a8bbf181a547dafdf5eb156135626728e00000000000000000000000000000000000000000000000000000000000138820000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default().try_into().unwrap();
        let msg = MsgMilestone::fromABIBytes(&msg_bytes);
        assert_eq!(msg.startBlock, 3887749);
        assert_eq!(msg.endBlock, 3887762);
        assert_eq!(hex::encode(msg.proposer), "4ad84f7014b7b44f723f284a85b1662337971439");
        assert_eq!(hex::encode(msg.hash), "6f73bdeda24c8d6b978628e10c425f5a8bbf181a547dafdf5eb156135626728e");
        assert_eq!(msg.borChainID, 80002);
    }
}