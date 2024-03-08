
function computeLocalLedger(

var res =  _.reduce(orderedWithdrawals, (withdrawal, memo) => {
    memo.merkleTree.write(encodeWithdrawalForTree(withdrawal)); // not sure how this is encoded
    memo.ledger[localChainId][withdrawal.assetId] = (memo.ledger[localChainId][withdrawal.assetId] || 0) - withdrawal.amount;
    memo.ledger[withdrawal.targetChainId][withdrawal.assetId] += (memo.ledger[withdrawal.targetChainId][withdrawal.assetId] || 0) + withdrawal.amount;
}, {ledger: {}, merkleTree: MerkleHasher()}) ;

res.merkeRoot = res.merkeTree.finish();

return res;
}