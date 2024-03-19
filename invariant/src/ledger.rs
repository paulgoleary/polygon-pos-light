use std::ops::{Add, Neg, Sub};
use num_bigint::{BigInt, Sign};
use reth_primitives::Address;
use reth_primitives::revm_primitives::HashMap;
use crate::invariant::Deposit;

struct Ledger<> {
    l: Vec<HashMap<Address, BigInt>>
}

impl Ledger<> {

    pub fn new(cnt_networks: usize) -> Ledger<> {
        Ledger {
            l: vec![HashMap::new(); cnt_networks]
        }
    }

    fn process_entry(&mut self, network_id: u32, asset_address: Address, amt: &BigInt, debit: bool) {
        match self.l.get_mut(network_id as usize) {
            Some(la) => {
                if debit {
                    *la.entry(asset_address).or_insert(BigInt::from(0)) -= amt;
                } else {
                    *la.entry(asset_address).or_insert(BigInt::from(0)) += amt;
                }
            }
            None => {
                panic!("ledger init'd for {} networks, got id {}", self.l.len(), network_id)
            }
        }
    }

    pub fn process_event(&mut self, dep: &Deposit, is_claim: bool) {
        //     memo.ledger[localChainId][withdrawal.assetId] = (memo.ledger[localChainId][withdrawal.assetId] || 0) - withdrawal.amount;
        //     memo.ledger[withdrawal.targetChainId][withdrawal.assetId] += (memo.ledger[withdrawal.targetChainId][withdrawal.assetId] || 0) + withdrawal.amount;
        self.process_entry(dep.orig_network, dep.orig_address, &dep.amount, is_claim);
        self.process_entry(dep.dest_network, dep.dest_address, &dep.amount, !is_claim);
    }

    pub fn network_balance(&self, network_id: u32) -> BigInt {
        match self.l.get(network_id as usize) {
            Some(la) => {
                return la.values().fold(BigInt::from(0), |acc, bal| acc + bal)
            }
            None => {
                panic!("ledger init'd for {} networks, got id {}", self.l.len(), network_id)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Neg;
    use num_bigint::BigInt;
    use crate::invariant::Deposit;
    use crate::ledger::Ledger;

    #[test]
    fn test_basic_ledger() {
        let mut l = Ledger::new(2);
        let d = Deposit::make_test(0);

        l.process_event(&d, false);
        assert_eq!(&l.network_balance(d.orig_network), &d.amount);
        assert_eq!(&l.network_balance(d.dest_network), &(&d.amount).neg());

        l.process_event(&d, true);
        assert_eq!(l.network_balance(d.orig_network), BigInt::from(0));
        assert_eq!(l.network_balance(d.dest_network), BigInt::from(0));
    }
}