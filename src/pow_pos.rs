use num::{CheckedAdd, CheckedSub, Zero};
use crate::{support::DispatchResult};
use std::collections::BTreeMap;


pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + PartialOrd+From<u128>;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub stake_pool: BTreeMap<T::AccountId, T::Balance>,
    pub mined_blocks: BTreeMap<T::BlockNumber, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            stake_pool: BTreeMap::new(),
            mined_blocks: BTreeMap::new(),
        }
    }

  
    pub fn stake(&mut self, staker: T::AccountId, amount: T::Balance) -> DispatchResult {
        if amount.is_zero() {
            return Err("Stake amount must be greater than zero");
        }
        let balance = self.stake_pool.entry(staker.clone()).or_insert_with(T::Balance::zero);
        *balance = balance.checked_add(&amount).ok_or("Overflow in staking balance")?;
        Ok(())
    }


    pub fn mine_block(
        &mut self,
        miner: T::AccountId,
        block_number: T::BlockNumber,
        proof_of_work: u64, 
    ) -> DispatchResult {
        if self.mined_blocks.contains_key(&block_number) {
            return Err("Block already mined");
        }

        let pow_valid = proof_of_work < 10_000; 

        let pos_valid = self
            .stake_pool
            .get(&miner)
            .map(|stake| *stake >= T::Balance::from(50u128))
            .unwrap_or(false); // Require at least 50 units staked

        if pow_valid && pos_valid {
            self.mined_blocks.insert(block_number, miner);
            Ok(())
        } else {
            Err("Block mining failed due to consensus validation")
        }
    }
}

pub enum Call<T: Config> {
    Stake { amount: T::Balance },
    MineBlock { block_number: T::BlockNumber, proof_of_work: u64 },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Call = Call<T>;
    type Caller = T::AccountId;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::Stake { amount } => self.stake(caller, amount),
            Call::MineBlock { block_number, proof_of_work } => {
                self.mine_block(caller, block_number, proof_of_work)
            }
        }
    }
}
