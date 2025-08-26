// programs/habitdao/src/lib.rs
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions ::*;
use state ::*;


declare_id!("BW5yXwViMeeYR2Ry9c8XcQrn9s5P4Y6CHv37c3F5zQ2w");

#[program]
pub mod habitdao {
    use super::*;

    pub fn initialize_dao(
        ctx: Context<initialize_dao::InitializeDao>,
        penalty_rate: u16,
        round_duration: i64,
    ) -> Result<()> {
        instructions::initialize_dao(ctx, penalty_rate, round_duration)
    }

    pub fn stake_tokens(
        ctx: Context<stake_tokens::StakeTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::stake_tokens(ctx, amount)
    }


}