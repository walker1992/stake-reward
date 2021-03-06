//! An StakingRewards-like Staking program for the Solana blockchain

pub mod processor;
pub mod instruction;
pub mod state;
pub mod error;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};


pub const ADD_SEED_TOKEN_ACCOUNT_AUTHORITY: &str = "TOKEN_ACCOUNT_AUTHORITY_test_8"; 
pub const BUMP_SEED_TOKEN_ACCOUNT_AUTHORITY: u8 = 255; 
 
pub const ADD_SEED_MASTER_STAKING: &str = "MASTER_STAKING_test_8";  
pub const BUMP_SEED_MASTER_STAKING: u8 = 254; 

pub const ADD_SEED_STATE_POOL: &str = "STATE_POOL";
pub const ADD_SEED_WALLET_POOL: &str = "WALLET_POOL"; // PDA with SOL for creating PDA UserInfo
pub const ADD_SEED_STAKED: &str = "STAKED"; // PDA t-a with staked tokens. Reward tokens are kept in other PDA t-a

solana_program::declare_id!("EyJ4ZNzAK8HJJrRbTTE6x769RA2h95zj826194DxyEbw");

/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(spl_token_program_id: &Pubkey) -> ProgramResult {
    if spl_token_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
