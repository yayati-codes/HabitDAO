use anchor_lang::prelude::*;

declare_id!("BW5yXwViMeeYR2Ry9c8XcQrn9s5P4Y6CHv37c3F5zQ2w");

#[program]
pub mod habitdao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
