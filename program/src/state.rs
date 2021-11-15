//! State Staking types

use solana_program::{
    program_pack::{
        IsInitialized,
        Sealed,
        Pack,
    },
    program_error::{
        ProgramError,
        PrintProgramError,
    },
    account_info::AccountInfo,
    program_option::COption,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    clock::Clock,
    msg,
};
use derivative::*;
use spl_token::state::Account as TokenAccount;
use arrayref::{
    array_refs,
    array_ref,
    array_mut_ref,
    mut_array_refs,
};
use borsh::{
    BorshDeserialize,
    BorshSerialize,
    BorshSchema,
};
use crate::error::StakingError;
use crate::utils::get_precision_factor;

pub const MASTER_STAKING_LEN: usize = 8;
pub const rewards_duration: u64 = 7 * 24 * 60 * 60;
pub const rewards_lock_duration: u64 = 1 * 24 * 60 * 60;

#[repr(C)]
#[derive(Debug, Clone, Copy, BorshSchema, BorshSerialize, BorshDeserialize)]
pub struct MasterStaking {
    pub pool_counter: u64,
}

impl MasterStaking {
    pub fn from_account_info(
        a: &AccountInfo
    ) -> Result<MasterStaking, ProgramError> {
        let master = MasterStaking::try_from_slice(
            &a.data.borrow_mut(),
        );
        let master = match master {
            Ok(v) => v,
            Err(_) => {
                StakingError::InvalidMasterStaking.print::<StakingError>();
                return Err(StakingError::InvalidMasterStaking.into());
            }
        };

        Ok(master)
    }

    pub fn increase_counter(
        &mut self,
    ) -> Result<(), ProgramError> {
        self.pool_counter = self.pool_counter
            .checked_add(1)
            .ok_or(StakingError::PoolCounterOverflow)?;

        Ok(())
    }
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct StakePool {
    pub pool_index: u64,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub is_initialized: u8,
    pub precision_factor_rank: u8,
    pub bonus_multiplier: COption<u8>,
    pub bonus_start_block: COption<u64>,
    pub bonus_end_block: COption<u64>,
    pub last_reward_block: u64,
    pub start_block: u64,
    pub end_block: u64,
    pub reward_amount: u64,
    pub reward_per_block: u64,
    pub accrued_token_per_share: u128,

    pub period_finish: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub reward_per_token_stored: u128,
    pub total_supply: u64,
}

impl Sealed for StakePool {}

impl IsInitialized for StakePool {
    fn is_initialized(&self) -> bool {
        self.is_initialized != 0
    }
}

impl Pack for StakePool {
    const LEN: usize = 321;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 215];
        let (
            pool_index,
            owner,
            mint,
            is_initialized,
            precision_factor_rank,
            bonus_multiplier,
            bonus_start_block,
            bonus_end_block,
            last_reward_block,
            start_block,
            end_block,
            reward_amount,
            reward_per_block,
            accrued_token_per_share,
            period_finish,
            reward_rate,
            last_update_time,
            reward_per_token_stored,
            total_supply,
        ) = array_refs![src, 8, 32, 32, 1, 1, 5, 12, 12, 8, 8, 8, 8, 8, 16, 8, 16, 8, 16, 8];
        Ok(StakePool {
            pool_index: u64::from_le_bytes(*pool_index),
            owner: Pubkey::new_from_array(*owner),
            mint: Pubkey::new_from_array(*mint),
            is_initialized: u8::from_le_bytes(*is_initialized),
            precision_factor_rank: u8::from_le_bytes(*precision_factor_rank),
            bonus_multiplier: unpack_coption_u8(bonus_multiplier)?,
            bonus_start_block: unpack_coption_u64(bonus_start_block)?,
            bonus_end_block: unpack_coption_u64(bonus_end_block)?,
            last_reward_block: u64::from_le_bytes(*last_reward_block),
            start_block: u64::from_le_bytes(*start_block),
            end_block: u64::from_le_bytes(*end_block),
            reward_amount: u64::from_le_bytes(*reward_amount),
            reward_per_block: u64::from_le_bytes(*reward_per_block),
            accrued_token_per_share: u128::from_le_bytes(*accrued_token_per_share),
            period_finish: u64::from_le_bytes(*period_finish),
            reward_rate: u128::from_le_bytes(*reward_rate),
            last_update_time: u64::from_le_bytes(*last_update_time),
            reward_per_token_stored: u128::from_le_bytes(*reward_per_token_stored),
            total_supply: u64::from_le_bytes(*total_supply),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 215];
        let (
            pool_index_dst,
            owner_dst,
            mint_dst,
            is_initialized_dst,
            precision_factor_rank_dst,
            bonus_multiplier_dst,
            bonus_start_block_dst,
            bonus_end_block_dst,
            last_reward_block_dst,
            start_block_dst,
            end_block_dst,
            reward_amount_dst,
            reward_per_block_dst,
            accrued_token_per_share_dst,
            period_finish,
            reward_rate,
            last_update_time,
            reward_per_token_stored,
        ) = mut_array_refs![dst, 8, 32, 32, 1, 1, 5, 12, 12, 8, 8, 8, 8, 8, 16, 8, 16, 8, 16,8];
        let &StakePool {
            pool_index,
            ref owner,
            ref mint,
            is_initialized,
            precision_factor_rank,
            ref bonus_multiplier,
            ref bonus_start_block,
            ref bonus_end_block,
            last_reward_block,
            start_block,
            end_block,
            reward_amount,
            reward_per_block,
            accrued_token_per_share,
            period_finish,
            reward_rate,
            last_update_time,
            reward_per_token_stored,
            total_supply,
        } = self;
        *pool_index_dst = pool_index.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        mint_dst.copy_from_slice(mint.as_ref());
        *is_initialized_dst = is_initialized.to_le_bytes();
        *precision_factor_rank_dst = precision_factor_rank.to_le_bytes();
        pack_coption_u8(bonus_multiplier, bonus_multiplier_dst);
        pack_coption_u64(bonus_start_block, bonus_start_block_dst);
        pack_coption_u64(bonus_end_block, bonus_end_block_dst);
        *last_reward_block_dst = last_reward_block.to_le_bytes();
        *start_block_dst = start_block.to_le_bytes();
        *end_block_dst = end_block.to_le_bytes();
        *reward_amount_dst = reward_amount.to_le_bytes();
        *reward_per_block_dst = reward_per_block.to_le_bytes();
        *accrued_token_per_share_dst = accrued_token_per_share.to_le_bytes();
        *period_finish = period_finish.to_le_bytes();
        *reward_rate = reward_rate.to_le_bytes();
        *last_update_time = last_update_time.to_le_bytes();
        *reward_per_token_stored = reward_per_token_stored.to_le_bytes();
        *total_supply = total_supply.to_le_bytes();
    }
}

impl StakePool {
    pub fn update_pool(
        &mut self,
        pda_pool_token_account_staked: &TokenAccount,
        clock: &Clock,
        amount: u64,
    ) -> ProgramResult {
        let current_block = clock.slot;
        if current_block <= self.last_reward_block {
            return Ok(());
        }

        let staked_token_supply = pda_pool_token_account_staked.amount;

        if staked_token_supply == 0 {
            self.set_last_reward_block(current_block);

            return Ok(());
        }

        let multiplier = self.get_multiplier(self.last_reward_block, current_block);

        let reward = multiplier
            .checked_mul(self.reward_per_block)
            .ok_or(StakingError::RewardOverflow)?;

        let precision_factor = get_precision_factor(
            self.precision_factor_rank,
        )?;

        self.accrued_token_per_share = self
            .accrued_token_per_share
            .checked_add(
                (reward as u128)
                    .checked_mul(precision_factor as u128)
                    .ok_or(StakingError::RewardMulPrecisionOverflow)?
                    .checked_div(staked_token_supply as u128)
                    .ok_or(StakingError::RewardMulPrecisionDivSupplyOverflow)?)
            .ok_or(StakingError::AccuredTokenPerShareOverflow)?;

        self.total_supply = self.total_supply
            .checked_add(amount).ok_or(StakingError::TotalSupplyOverflow)?;

        //debug
        msg!(
         "multiplier: {}\n
         reward: {}\n
         staked_token_supply: {}\n,
         accrued_toked: {}\n",
         multiplier,
         reward,
         self.total_supply,
         self.accrued_token_per_share,
      );
        //

        if self.end_block > current_block {
            self.set_last_reward_block(current_block);
        } else {
            self.set_last_reward_block(self.end_block);
        }

        if let COption::Some(v) = self.bonus_end_block {
            if v != 0 && current_block > v {
                self.bonus_start_block = COption::None;
                self.bonus_end_block = COption::None;
                self.set_bonus_multiplier(1);
            }
        }

        Ok(())
    }

    fn get_multiplier(
        &self,
        mut from: u64,
        mut to: u64,
    ) -> u64 {
        if from < self.start_block {
            from = self.start_block;
        }
        if self.end_block < to {
            to = self.end_block;
        }

        let multiplier: u64 = self.bonus_multiplier.unwrap().into();
        let start = match self.bonus_start_block {
            COption::Some(v) => v,
            COption::None => 0,
        };
        let end = match self.bonus_end_block {
            COption::Some(v) => v,
            COption::None => 0,
        };

        if from < start && to > end {
            return start - from + to - end + (end - start) * multiplier;
        } else if from < start && to > start {
            return start - from + (to - start) * multiplier;
        } else if from < end && to > end {
            return to - end + (end - from) * multiplier;
        } else if from >= start && to <= end {
            return (to - from) * multiplier;
        } else {
            return to - from;
        }
    }

    fn set_last_reward_block(
        &mut self,
        block: u64,
    ) {
        self.last_reward_block = block;
    }

    pub fn set_end_block(
        &mut self,
        block: u64,
    ) {
        self.end_block = block;
    }

    pub fn set_bonus_multiplier(
        &mut self,
        multiplier: u8,
    ) {
        self.bonus_multiplier = COption::Some(multiplier);
    }

    pub fn set_last_update_time(
        &mut self,
        last_update_time: u64,
    ) {
        self.last_update_time = last_update_time;
    }


    pub fn updateReward(
        &mut self,
        clock: &Clock,
    ) {
     let reward_per_token = self.get_reward_per_token(clock);
     let last_update_time = self.get_last_time_reward_applicable(clock);


    }


    pub fn get_last_time_reward_applicable(
        &mut self,
        clock: &Clock,
    ) -> u64 {
        let period_finish = match self.period_finish {
            COption::Some(v) => v,
            COption::None => 0,
        };

        return if clock.unixTimestamp < period_finish {
            period_finish
        } else {
            clock.unixTimestamp
        };
    }

    pub fn get_reward_per_token(
        &mut self,
        clock: &Clock,
    ) -> u128 {
        let total_supply :u64 = match self.total_supply {
            COption::Some(v) => v,
            COption::None => 0,
        };

        let reward_per_token_stored :u128 = match self.reward_per_token_stored {
            COption::Some(v) => v,
            COption::None => 0,
        };

        if total_supply == 0 {
            return reward_per_token_stored;
        }

        let last_update_time :u64 = match self.last_update_time {
            COption::Some(v) => v,
            COption::None => 0,
        };

        let reward_rate :u64 = match self.reward_rate {
            COption::Some(v) => v,
            COption::None => 0,
        };

        let precision_factor :u64 = match self.precision_factor_rank {
            COption::Some(v) => v,
            COption::None => 0,
        };


        let last_time_reward_applicable = self.get_last_time_reward_applicable(clock);

        let last_reward_per_token_stored = reward_per_token_stored
            .checked_add(
                last_time_reward_applicable.checked_sub(last_update_time).ok_or(StakingError::RewardOverflow)?
            .checked_mul(reward_rate).ok_or(StakingError::RewardOverflow)?
            .checked_mul(precision_factor).ok_or(StakingError::RewardOverflow)?
            .checked_div(total_supply).ok_or(StakingError::RewardOverflow)?
                    as u128
            ).ok_or(StakingError::RewardOverflow)?;

        return last_reward_per_token_stored;
    }


}

pub const USER_INFO_LEN: usize = 48;

#[repr(C)]
#[derive(Debug, Copy, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserInfo {
    pub token_account_id: Pubkey,
    pub amount: u64,
    pub reward_debt: u64,
    pub reward_lock_finish: u64,
}

impl UserInfo {
    pub fn from_account_info(
        a: &AccountInfo
    ) -> Result<UserInfo, ProgramError> {
        let user_info = UserInfo::try_from_slice(
            &a.data.borrow_mut(),
        );
        let user_info = match user_info {
            Ok(v) => v,
            Err(_) => {
                StakingError::InvalidUserInfo.print::<StakingError>();
                return Err(StakingError::InvalidUserInfo.into());
            }
        };

        Ok(user_info)
    }

    pub fn set_reward_debt(
        &mut self,
        value: u64,
    ) {
        self.reward_debt = value;
    }

    pub fn set_reward_lock_finish(
        &mut self,
        clock: &Clock,
    ) {
        let current_time = clock.unixTimestamp;
        self.reward_lock_finish = current_time
            .checked_add(rewards_lock_duration)
            .ok_or(StakingError::Overflow)?;
    }
}

fn unpack_coption_u8(src: &[u8; 5]) -> Result<COption<u8>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 1];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u8::from_le_bytes(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}

fn pack_coption_u8(src: &COption<u8>, dst: &mut [u8; 5]) {
    let (tag, body) = mut_array_refs![dst, 4, 1];
    match src {
        COption::Some(amount) => {
            *tag = [1, 0, 0, 0];
            *body = amount.to_le_bytes();
        }
        COption::None => {
            *tag = [0; 4];
        }
    }
}

fn unpack_coption_u64(src: &[u8; 12]) -> Result<COption<u64>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 8];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}

fn pack_coption_u64(src: &COption<u64>, dst: &mut [u8; 12]) {
    let (tag, body) = mut_array_refs![dst, 4, 8];
    match src {
        COption::Some(amount) => {
            *tag = [1, 0, 0, 0];
            *body = amount.to_le_bytes();
        }
        COption::None => {
            *tag = [0; 4];
        }
    }
}