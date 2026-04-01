use anchor_lang::prelude::*;

declare_id!("7kyhzDhep8bNzvcYbirc8jdPgFYEkLwfBnXEFQ2oFAzu");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
