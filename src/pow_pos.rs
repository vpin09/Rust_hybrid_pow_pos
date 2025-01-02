use num::{CheckedAdd, CheckedSub, Zero};

use crate::{support::DispatchResult};
use core::fmt::Debug;
use std::collections::BTreeMap;


pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub stake_pool: BTreeMap<T::AccountId, T::Balance>,
    pub mined_blocks: BTreeMap<T::BlockNumber, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            stake_pool:BTreeMap::new(),
            mined_blocks: BTreeMap::new(),
        }
    }

    pub fn mine_block(
        &mut self,
        miner: T::AccountId,
        block_number: T::BlockNumber,
    ) -> DispatchResult {
        if self.mined_blocks.contains_key(&block_number) {
            return Err("Block already mined");
        }

        self.mined_blocks.insert(block_number, miner);
        Ok(())
    }

    pub fn stake(&mut self, staker: T::AccountId, amount: T::Balance) -> DispatchResult {
        let balance = self.stake_pool.entry(staker.clone()).or_insert(T::Balance::zero());
        balance.checked_add(&amount);
        Ok(())
    }

   
}

pub enum Call<T: Config> {
    MineBlock { block_number: T::BlockNumber },
    Stake { amount: T::Balance },
}
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> DispatchResult {
        match call {
            Call::MineBlock { block_number } => {
                self.mine_block(caller, block_number)?;
            }
            Call::Stake { amount } => {
                self.stake(caller, amount)?;
            }
        }
        Ok(())
    }
}
