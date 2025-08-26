// programs/habitdao/src/instructions/stake_tokens.rs
use anchor_lang::prelude::*;
use crate::state ::*;
use anchor_spl::{
associated_token::AssociatedToken,
token_2022::Token2022,
token_interface::{Mint, TokenAccount, TokenInterface, Transfer, transfer},
};
use crate::state::*;

#[derive(Accounts)]
pub struct StakeTokens<'info> {
#[account(mut)]
pub member: Signer<'info>,


// DAO config
#[account(
    seeds = [b"dao_config"],
    bump
)]
pub dao_config: Account<'info, DaoConfig>,

// Member profile
#[account(
    init_if_needed,
    payer = member,
    space = MemberProfile::LEN,
    seeds = [b"member", member.key().as_ref()],
    bump
)]
pub member_profile: Account<'info, MemberProfile>,

// Token mint (must match DAO config)
#[account(
    constraint = mint.key() == dao_config.mint
)]
pub mint: InterfaceAccount<'info, Mint>,

// Member's token account
#[account(
    init_if_needed,
    payer = member,
    associated_token::mint = mint,
    associated_token::authority = member,
    associated_token::token_program = token_program,
)]
pub member_token_account: InterfaceAccount<'info, TokenAccount>,

// Vault authority PDA
/// CHECK: PDA for vault authority
#[account(
    seeds = [b"vault_authority", dao_config.key().as_ref()],
    bump = dao_config.vault_authority_bump
)]
pub vault_authority: UncheckedAccount<'info>,

// Vault token account
#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = vault_authority,
    associated_token::token_program = token_program,
)]
pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

pub token_program: Program<'info, Token2022>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,



}

pub fn stake_tokens(
ctx: Context<StakeTokens>,
amount: u64,
) -> Result<()> {
let member_profile = &mut ctx.accounts.member_profile;


// Initialize member profile if first time
if member_profile.authority == Pubkey::default() {
    member_profile.authority = ctx.accounts.member.key();
    member_profile.staked_amount = 0;
    member_profile.last_checkin = 0;
    member_profile.streak_count = 0;
    member_profile.total_rewards = 0;
    member_profile.is_active = false;
}

// Transfer tokens from member to vault
let transfer_accounts = Transfer {
    from: ctx.accounts.member_token_account.to_account_info(),
    to: ctx.accounts.vault_token_account.to_account_info(),
    authority: ctx.accounts.member.to_account_info(),
};

let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    transfer_accounts,
);

transfer(cpi_ctx, amount)?;

// Update member profile
member_profile.staked_amount += amount;
member_profile.is_active = true;

msg!(
    "Member {} staked {} tokens. Total staked: {}",
    ctx.accounts.member.key(),
    amount,
    member_profile.staked_amount
);

Ok(())


}