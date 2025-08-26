// programs/habitdao/src/state.rs
use anchor_lang::prelude::*;

#[account]
pub struct DaoConfig {
pub mint: Pubkey,                    // Token-2022 mint address
pub vault_authority_bump: u8,        // PDA bump for vault authority
pub admin: Pubkey,                   // DAO admin/governance
pub penalty_rate: u16,               // Penalty percentage (basis points)
pub reward_pool: u64,                // Current reward pool balance
pub current_round: u64,              // Current habit tracking round
pub round_duration: i64,             // Round duration in seconds
}

impl DaoConfig {
pub const LEN: usize = 8 + // discriminator
32 + // mint
1 +  // vault_authority_bump
32 + // admin
2 +  // penalty_rate
8 +  // reward_pool
8 +  // current_round
8;   // round_duration
}

#[account]
pub struct MemberProfile {
pub authority: Pubkey,               // Member's wallet
pub staked_amount: u64,              // Current staked tokens
pub last_checkin: i64,               // Last check-in timestamp
pub streak_count: u32,               // Current streak
pub total_rewards: u64,              // Lifetime rewards earned
pub is_active: bool,                 // Active in current round
}

impl MemberProfile {
pub const LEN: usize = 8 + // discriminator
32 + // authority
8 +  // staked_amount
8 +  // last_checkin
4 +  // streak_count
8 +  // total_rewards
1;   // is_active
}