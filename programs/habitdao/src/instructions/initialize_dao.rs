// programs/habitdao/src/instructions/initialize_dao.rs
use anchor_lang::prelude::*;
use crate::state ::*;
use anchor_spl::{
associated_token::AssociatedToken,
token_2022::{Token2022},
token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeDao<'info> {
#[account(mut)]
pub admin: Signer<'info>,


// DAO config account - stores mint and settings
#[account(
    init,
    payer = admin,
    space = DaoConfig::LEN,
    seeds = [b"dao_config"],
    bump
)]
pub dao_config: Account<'info, DaoConfig>,

// Token-2022 mint (must already exist)
pub mint: InterfaceAccount<'info, Mint>,

// PDA that will own the vault token account
/// CHECK: PDA derived for vault authority
#[account(
    seeds = [b"vault_authority", dao_config.key().as_ref()],
    bump
)]
pub vault_authority: UncheckedAccount<'info>,

// Vault token account owned by the PDA
#[account(
    init,
    payer = admin,
    associated_token::mint = mint,
    associated_token::authority = vault_authority,
    associated_token::token_program = token_program,
)]
pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

// Token-2022 program
pub token_program: Program<'info, Token2022>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,



}

pub fn initialize_dao(
ctx: Context<InitializeDao>,
penalty_rate: u16,
round_duration: i64,
) -> Result<()> {
let dao_config = &mut ctx.accounts.dao_config;
let vault_authority_bump = ctx.bumps.vault_authority;

// Validate Token-2022 program
require_eq!(
    ctx.accounts.token_program.key(),
    anchor_spl::token_2022::ID,
    ErrorCode::InvalidTokenProgram
);

// Initialize DAO config
dao_config.mint = ctx.accounts.mint.key();
dao_config.vault_authority_bump = vault_authority_bump;
dao_config.admin = ctx.accounts.admin.key();
dao_config.penalty_rate = penalty_rate;
dao_config.reward_pool = 0;
dao_config.current_round = 1;
dao_config.round_duration = round_duration;

msg!(
    "DAO initialized with Token-2022 mint: {}, vault: {}",
    dao_config.mint,
    ctx.accounts.vault_token_account.key()
);

Ok(())



}

#[error_code]
pub enum ErrorCode {
#[msg("Invalid token program - must use Token-2022")]
InvalidTokenProgram,
}