use anchor_lang::prelude::*;

declare_id!("Cgy2rqHmchiEd74c5EXz4ZusQkMSXc8rDBdz8TmoBFzZ");

#[program]
pub mod transfer_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
