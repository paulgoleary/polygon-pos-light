use sha3::digest::Update;
use sha3::{Digest, Keccak256};
use crate::hasher::KeccakAlgorithm;

pub struct CheckpointHeader {
    number: u64,
    timestamp: u64,
    txRoot: [u8; 32],
    rcptRoot: [u8; 32]
}

impl CheckpointHeader {
    fn new(number: u64, timestamp: u64, txRoot: [u8; 32], rcptRoot: [u8; 32]) -> Self {
        CheckpointHeader{
            number,
            timestamp,
            txRoot,
            rcptRoot,
        }
    }

    // hmmm... not sure this does anything i need ...?
    fn hash32(&self) -> [u8; 32] {
        let mut be_num = [0 as u8; 32];
        be_num[24..].copy_from_slice(&self.number.to_be_bytes());

        let mut be_ts = [0 as u8; 32];
        be_ts[24..].copy_from_slice(&self.timestamp.to_be_bytes());

        let mut hasher = Keccak256::new();
        sha3::Digest::update(&mut hasher, be_num);
        sha3::Digest::update(&mut hasher, be_ts);
        sha3::Digest::update(&mut hasher, self.txRoot);
        sha3::Digest::update(&mut hasher, self.rcptRoot);

        <[u8; 32]>::from(hasher.finalize())
    }
}

#[cfg(test)]
mod test {
    use crate::checkpoint::CheckpointHeader;

    #[test]
    fn test_basic() {
        let root_hash = hex::decode("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421").unwrap_or_default().try_into().unwrap();
        let header = CheckpointHeader{
            number: 3639411,
            timestamp: 1708274440,
            txRoot: root_hash,
            rcptRoot: root_hash,
        };
        let test_hash = header.hash32();
        let expect_hash: [u8; 32] = hex::decode("fc905b8816642b177111968433a6aea8ea790ad2ea7c164de1625eaf01270f88").unwrap_or_default().try_into().unwrap();
        assert_eq!(test_hash, expect_hash)
    }
}